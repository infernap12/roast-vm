use crate::class::RuntimeClass;
use crate::class_file::{ClassFile, MethodData, MethodRef};
use crate::class_loader::{ClassLoader, LoaderRef};
use crate::jni::create_jni_function_table;
use crate::objects::object::{ObjectReference, ReferenceKind};
use crate::objects::object_manager::ObjectManager;
use crate::value::{Primitive, Value};
use crate::vm::Vm;
use crate::{BaseType, FieldType, Frame, MethodDescriptor, ThreadId};
use deku::DekuError::Incomplete;
use itertools::Itertools;
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jobject, jshort, JNIEnv};
use libffi::low::call;
use libffi::middle::*;
use log::{trace, warn};
use std::any::Any;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::{Add, Deref};
use std::ptr::null_mut;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::vec::IntoIter;
use crate::error::VmError;

type MethodCallResult = Result<Option<Value>, VmError>;

// Thread-local storage for current thread ID
// In single-threaded mode: stores the one thread ID
// In multi-threaded mode: each OS thread has its own thread ID
thread_local! {
	static CURRENT_THREAD_ID: RefCell<Option<ThreadId>> = RefCell::new(None);
}

// A thread of execution
pub struct VmThread {
	pub id: ThreadId,
	pub vm: Arc<Vm>,
	pub loader: Arc<Mutex<ClassLoader>>,
	pub frame_stack: Mutex<Vec<Arc<Mutex<Frame>>>>,
	pub gc: Arc<RwLock<ObjectManager>>,
	pub jni_env: JNIEnv,
}

impl VmThread {
	pub fn new(vm: Arc<Vm>, loader: Option<LoaderRef>) -> Arc<Self> {
		static NEXT_ID: AtomicU64 = AtomicU64::new(0);
		let id = ThreadId(NEXT_ID.fetch_add(1, Ordering::SeqCst));

		let loader = loader.unwrap_or(vm.loader.clone());
		let gc = vm.gc.clone();
		Arc::new_cyclic(|weak_self| {
			let jni_env = create_jni_function_table(weak_self.as_ptr() as *mut VmThread);
			Self {
				id,
				vm,
				loader,
				frame_stack: Default::default(),
				gc,
				jni_env,
			}
		})
	}

	/// Get current thread ID from thread-local storage
	pub fn current_id() -> ThreadId {
		CURRENT_THREAD_ID.with(|cell| cell.borrow().expect("No current thread set"))
	}

	/// Set current thread ID for this OS thread
	pub fn set_current(id: ThreadId) {
		CURRENT_THREAD_ID.with(|cell| {
			*cell.borrow_mut() = Some(id);
		});
	}

	/// Get current thread from VM using thread-local storage
	pub fn current(vm: &Arc<Vm>) -> Arc<VmThread> {
		let id = Self::current_id();
		vm.threads.get(&id).unwrap().clone()
	}

	/// Get or resolve a class, ensuring it and its dependencies are initialized.
	/// Follows JVM Spec 5.5 for recursive initialization handling.
	pub fn get_or_resolve_class(&self, what: &str) -> Result<Arc<RuntimeClass>, VmError> {
		// Phase 1: Load the class (short lock)
		let runtime_class = self
			.loader
			.lock()
			.unwrap()
			.get_or_load(what, None)?;

		// Phase 2: Collect classes that need initialisation (short lock)
		let classes_to_init = {
			let mut loader = self.loader.lock().unwrap();
			let classes = loader.needs_init.clone();
			loader.needs_init.clear();
			classes
		};

		// Phase 3: Initialise each class (NO lock held - allows recursion)
		for class in classes_to_init {
			self.init(class)?;
		}

		Ok(runtime_class)
	}

	pub fn get_class(&self, what: &str) -> Result<Arc<RuntimeClass>, VmError> {
		self.loader
			.lock()
			.unwrap()
			.get_or_load(what, None)
	}

	/// Initialize a class following JVM Spec 5.5.
	/// Handles recursive initialization by tracking which thread is initializing.
	fn init(&self, class: Arc<RuntimeClass>) -> Result<(), VmError> {
		use crate::class::InitState;
		use std::thread;

		let current_thread = thread::current().id();

		// Check and update initialization state
		{
			let mut state = class.init_state.lock().unwrap();
			match &*state {
				InitState::Initialized => {
					// Already initialized, nothing to do
					return Ok(());
				}
				InitState::Initializing(tid) if *tid == current_thread => {
					// JVM Spec 5.5: Recursive initialization by same thread is allowed
					warn!(
						"Class {} already being initialized by this thread (recursive)",
						class.this_class
					);
					return Ok(());
				}
				InitState::Initializing(tid) => {
					// Different thread is initializing - in a real JVM we'd wait
					// For now, just return an error
					return Err(VmError::LoaderError(format!(
						"Class {} is being initialized by another thread",
						class.this_class
					)));
				}
				InitState::Error(msg) => {
					return Err(VmError::LoaderError(format!(
						"Class {} initialization previously failed: {}",
						class.this_class, msg
					)));
				}
				InitState::NotInitialized => {
					// Mark as being initialized by this thread
					*state = InitState::Initializing(current_thread);
				}
			}
		}
		let class_class = self.get_class("java/lang/Class")?;
		let string = self.intern_string(&class.this_class);
		let class_obj = self.gc.write().unwrap().new_class(
			class_class,
			Some(ReferenceKind::ObjectReference(string)),
			None,
			class.access_flags,
			false
		);
		let id = class_obj.lock().unwrap().id;
		class.mirror.set(id).expect("woops, id already set");
		// Perform actual initialisation
		trace!("Initializing class: {}", class.this_class);
		let result = (|| {
			// Initialize superclass first (if any)
			if let Some(ref super_class) = class.super_class {
				self.init(super_class.clone())?;
			}

			// Run <clinit> if present
			if let Ok(method) = class.find_method("<clinit>", &MethodDescriptor::void()) {
				self.execute_method(&class, &method, vec![])?;
			}
			Ok(())
		})();

		// Update state based on result
		{
			let mut state = class.init_state.lock().unwrap();
			match result {
				Ok(_) => {
					*state = InitState::Initialized;
					trace!("Class {} initialized successfully", class.this_class);
				}
				Err(ref e) => {
					*state = InitState::Error(format!("{:?}", e));
				}
			}
		}

		result
	}

	pub fn invoke_main(&self, what: &str) -> Result<(), VmError> {
		let method_ref = MethodRef {
			class: what.to_string(),
			name: "main".to_string(),
			desc: MethodDescriptor::psvm(),
		};

		self.invoke(method_ref, Vec::new())?;
		Ok(())
	}

	pub fn invoke(&self, method_reference: MethodRef, args: Vec<Value>) -> MethodCallResult {
		if method_reference.class.contains("Unsafe") {
			println!("()")
		}
		let class = self.get_or_resolve_class(&method_reference.class)?;
		let method = class.find_method(&method_reference.name, &method_reference.desc).unwrap();
		self.execute_method(&class, &method, args)
	}

	pub fn invoke_virtual(&self, method_reference: MethodRef, class: Arc<RuntimeClass>, args: Vec<Value>) -> MethodCallResult {
		let method = class.find_method(&method_reference.name, &method_reference.desc).unwrap();
		self.execute_method(&class, &method, args)
	}

	/*pub fn invoke_old(&self, method_reference: MethodRef, mut args: Vec<Value>) -> MethodCallResult {
		let mut new_args = Vec::new();
		let class = self.get_or_resolve_class(&method_reference.class)?;
		let resolved_method = class
			.find_method(&method_reference.name, &method_reference.desc)
			.unwrap();
		trace!(
			"invoking '{}' from {}",
			method_reference.name,
			class.this_class
		);
		if resolved_method.flags.ACC_NATIVE {
			if resolved_method.flags.ACC_STATIC {
				let jclass = self.vm.gc.read().unwrap().get(*class.mirror.wait());
				new_args.push(Value::Reference(Some(jclass)));
			}
			for arg in args {
				new_args.push(arg)
			}
			return self.invoke_native(&method_reference, new_args);
		}
		let mut frame = Frame::new(
			method_reference.clone(),
			resolved_method.code.clone().unwrap(),
			class.constant_pool.clone(),
			args,
			self.vm.clone(),
		);
		self.frame_stack.lock().unwrap().push(frame.clone());
		let result = frame.execute()?;
		self.frame_stack.lock().unwrap().pop();
		Ok(result)
	}*/

	pub fn invoke_native(&self, method: &MethodRef, mut args: Vec<Value>) -> MethodCallResult {
		let symbol_name = generate_jni_method_name(method, false);
		trace!("searching for native symbol: {:?}", &symbol_name);

		if symbol_name.contains("Java_java_lang_Class_desiredAssertionStatus0") {
			warn!("MAJOR HACK, figure out what is wrong with desiredAssertionStatus0");
			return Ok(Some(Value::from(false)));
		}

		let result = unsafe {
			let p = self
				.vm
				.find_native_method(&symbol_name)
				.or_else(|| {
					let name_with_params = generate_jni_method_name(method, true);
					self.vm.find_native_method(&name_with_params)
				})
				.ok_or(VmError::NativeError(format!("Link error: Unable to locate symbol {symbol_name}")))?;
			// build pointer to native fn
			let cp = CodePtr::from_ptr(p);

			// let args = build_args(args);

			// coerce my method descriptors into libffi C equivalents, then call
			// let l = method.build_cif().call::<jlong>(cp, args.as_ref());

			let mut storage = Vec::new();
			trace!("passing {} to native fn", Value::format_vec(&args));
			let deq_args = VecDeque::from(args);

			let built_args = build_args(
				deq_args,
				&mut storage,
				&self.jni_env as *const _ as *mut JNIEnv,
			);
			let cif = method.build_cif();

			match &method.desc.return_type {
				None => {
					cif.call::<()>(cp, built_args.as_ref());
					Ok(None)
				}
				Some(FieldType::Base(BaseType::Long)) => {
					let v = cif.call::<jlong>(cp, built_args.as_ref());
					Ok(Some(Value::Primitive(Primitive::Long(v))))
				}
				Some(FieldType::Base(BaseType::Int)) => {
					let v = cif.call::<jint>(cp, built_args.as_ref());
					Ok(Some(v.into()))
				}
				Some(FieldType::Base(BaseType::Float)) => {
					let v = cif.call::<jfloat>(cp, built_args.as_ref());
					Ok(Some(v.into()))
				}
				Some(FieldType::Base(BaseType::Double)) => {
					let v = cif.call::<jdouble>(cp, built_args.as_ref());
					Ok(Some(v.into()))
				}
				Some(FieldType::Base(BaseType::Boolean)) => {
					let v = cif.call::<jboolean>(cp, built_args.as_ref());
					Ok(Some(v.into()))
				}
				Some(FieldType::Base(BaseType::Byte)) => {
					let v = cif.call::<jbyte>(cp, built_args.as_ref());
					Ok(Some(v.into()))
				}
				Some(FieldType::Base(BaseType::Char)) => {
					let v = cif.call::<jchar>(cp, built_args.as_ref());
					Ok(Some(v.into()))
				}
				Some(FieldType::Base(BaseType::Short)) => {
					let v = cif.call::<jshort>(cp, built_args.as_ref());
					Ok(Some(v.into()))
				}
				Some(FieldType::ClassType(_)) | Some(FieldType::ArrayType(_)) => {
					let v = cif.call::<jobject>(cp, built_args.as_ref());
					// Convert jobject (u32 ID) to Reference
					let obj_id = v as u32;
					if obj_id == 0 {
						// Null reference
						Ok(Some(Value::Reference(None)))
					} else {
						// Look up the object in the ObjectManager
						let gc = self.gc.read().unwrap();
						let reference_kind = gc.get(obj_id);
						Ok(Some(Value::Reference(Some(reference_kind))))
					}
				}
			}
		};
		warn!("Invoke native not final");
		result
	}
	fn execute_method(
		&self,
		class: &Arc<RuntimeClass>,
		method: &MethodData,
		args: Vec<Value>,
	) -> MethodCallResult {
		eprintln!("[DEBUG] execute_method self.id = {:?}", self.id);
		let method_ref = MethodRef {
			class: class.this_class.clone(),
			name: method.name.clone(),
			desc: method.desc.clone(),
		};


		if method.flags.ACC_NATIVE {
			let mut native_args = Vec::new();
			if method.flags.ACC_STATIC {
				let jclass = self.vm.gc.read().unwrap().get(*class.mirror.wait());
				native_args.push(Value::Reference(Some(jclass)));
			}
			native_args.extend(args);
			return self.invoke_native(&method_ref, native_args);
		}

		let mut frame = Frame::new(
			class.clone(),
			method_ref,
			method.code.clone().unwrap(),
			class.constant_pool.clone(),
			args,
			self.vm.clone(),
			method.line_number_table.clone(),
		);
		let frame = Arc::new(Mutex::new(frame));
		self.frame_stack.lock().unwrap().push(frame.clone());
		eprintln!("[DEBUG] pushed frame for {}.{}, stack depth now: {}",
				  class.this_class, method.name,
				  self.frame_stack.lock().unwrap().len());
		let result = frame.lock().unwrap().execute();
		eprintln!("[DEBUG] returned from {}.{}, result ok: {}, stack depth: {}",
				  class.this_class, method.name, result.is_ok(),
				  self.frame_stack.lock().unwrap().len());
		if result.is_ok() {
			self.frame_stack.lock().unwrap().pop();
		}
		result
	}
	pub fn print_stack_trace(&self) {
		let guard = self.frame_stack.lock().unwrap();
		// Reverse - most recent frame first (like Java does)
		for frame in guard.iter().rev() {
			let frame = frame.lock().unwrap();
			let method = &frame.method_ref;
			// Internal format uses '/', Java stack traces use '.'
			let class_name = method.class.replace("/", ".");


			match (&frame.class.source_file, &frame.current_line_number()) {
				(Some(file), Some(line)) => eprintln!("\tat {}.{}({}:{})", class_name, method.name, file, line),
				(Some(file), None) => eprintln!("\tat {}.{}({})", class_name, method.name, file),
				_ => eprintln!("\tat {}.{}(Unknown Source)", class_name, method.name),
			}
		}
	}
}

fn build_args<'a>(
	mut params: VecDeque<Value>,
	storage: &'a mut Vec<Box<dyn Any>>,
	jnienv: *mut JNIEnv,
) -> Vec<Arg<'a>> {
	// Slot 0: JNIEnv
	storage.push(Box::new(jnienv));

	// Slot 1: this (instance) or class (static) — first param either way
	let receiver = params.pop_front();
	let receiver_id = match receiver {
		Some(Value::Reference(Some(ReferenceKind::ObjectReference(ref_kind)))) => {
			ref_kind.lock().unwrap().id
		} // however you get the u32 ID
		Some(Value::Reference(None)) => 0, // null
		_ => panic!("first arg must be reference"),
	};
	storage.push(Box::new(receiver_id as jobject));

	for value in params {
		match value {
			Value::Primitive(Primitive::Int(x)) => storage.push(Box::new(x) as Box<dyn Any>),
			Value::Primitive(Primitive::Boolean(x)) => storage.push(Box::new(x) as Box<dyn Any>),
			Value::Primitive(Primitive::Char(x)) => storage.push(Box::new(x) as Box<dyn Any>),
			Value::Primitive(Primitive::Float(x)) => storage.push(Box::new(x) as Box<dyn Any>),
			Value::Primitive(Primitive::Double(x)) => storage.push(Box::new(x) as Box<dyn Any>),
			Value::Primitive(Primitive::Byte(x)) => storage.push(Box::new(x) as Box<dyn Any>),
			Value::Primitive(Primitive::Short(x)) => storage.push(Box::new(x) as Box<dyn Any>),
			Value::Primitive(Primitive::Long(x)) => storage.push(Box::new(x) as Box<dyn Any>),
			Value::Reference(x) => {
				let id = x.map(|r| r.id()).unwrap_or(0) as jobject;
				storage.push(Box::new(id));
			}
			Value::Padding => { panic!("Uhh not possible chief") }
		}
	}

	// Create args referencing the storage
	storage.iter().map(|boxed| arg(&**boxed)).collect()
}

pub fn generate_jni_method_name(method_ref: &MethodRef, with_type: bool) -> String {
	let class_name = jni_escape(&method_ref.class);
	let method_name = jni_escape(&method_ref.name);

	let mut name = format!("Java_{class_name}_{method_name}");
	if with_type {
		let params = jni_escape(&method_ref.desc.param_string());
		let str = format!("__{}", params);
		name.push_str(&str)
	}
	name
}

pub fn jni_escape_char(c: char) -> String {
	match c {
		'/' => "_".to_string(),
		'_' => "_1".to_string(),
		';' => "_2".to_string(),
		'[' => "_3".to_string(),
		c if c.is_ascii_alphanumeric() => c.to_string(),
		c => format!("_0{:04x}", c as u32),
	}
}
pub fn jni_escape(s: &str) -> String {
	s.chars().map(jni_escape_char).join("")
}

impl From<FieldType> for Type {
	fn from(value: FieldType) -> Self {
		match value {
			FieldType::Base(v) => match v {
				BaseType::Byte => Type::i8(),
				BaseType::Char => Type::u16(),
				BaseType::Double => Type::f64(),
				BaseType::Float => Type::f32(),
				BaseType::Int => Type::i32(),
				BaseType::Long => Type::i64(),
				BaseType::Short => Type::i16(),
				BaseType::Boolean => Type::i8(),
			},
			FieldType::ClassType(_) => Self::pointer(),
			FieldType::ArrayType(_) => Self::pointer(),
		}
	}
}

impl MethodRef {
	fn build_cif(&self) -> Cif {
		let mut args = vec![
			Type::pointer(), //JNIEnv*
			Type::pointer(), //jclass
		];
		for v in self.desc.parameters.clone() {
			args.push(v.into())
		}
		let return_type = if let Some(x) = self.desc.return_type.clone() {
			x.into()
		} else {
			Type::void()
		};
		Builder::new().args(args).res(return_type).into_cif()
	}
}

impl VmThread {
	/// perhaps misleadingly named
	/// was once called get_or_make_string
	/// it will look for an already created string, and if its exists, return it
	/// if not, will cause a new String to be made, which at the time always interns it
	pub fn intern_string(&self, utf: &str) -> ObjectReference {
		// Fast path: read lock
		if let Some(existing) = self.gc.read().unwrap().get_interned_string(utf) {
			return existing;
		}

		// Slow path: write lock with re-check
		let mut gc = self.gc.write().unwrap();

		// Another thread may have inserted while we waited for the write lock
		if let Some(existing) = gc.get_interned_string(utf) {
			return existing;
		}

		let string_class = self.get_class("java/lang/String").unwrap();
		let byte_array_class = self.get_class("[B").unwrap();
		gc.new_string(byte_array_class, string_class, utf)
	}
}
