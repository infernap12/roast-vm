use crate::class::RuntimeClass;
use crate::class_file::{ClassFile, MethodData, MethodRef};
use crate::class_loader::{ClassLoader, LoaderRef};
use crate::jni::create_jni_function_table;
use crate::objects::object_manager::ObjectManager;
use crate::value::{Primitive, Value};
use crate::vm::Vm;
use crate::{BaseType, FieldType, Frame, MethodDescriptor, VmError};
use deku::DekuError::Incomplete;
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jobject, jshort};
use jni::JNIEnv;
use libffi::low::call;
use libffi::middle::*;
use log::{trace, warn};
use std::any::Any;
use std::ops::Add;
use std::ptr::null_mut;
use std::sync::{Arc, Mutex, RwLock};
use std::vec::IntoIter;

type MethodCallResult = Result<Option<Value>, VmError>;

// A thread of execution
pub struct VmThread {
	pub vm: Arc<Vm>,
	pub loader: Arc<Mutex<ClassLoader>>,
	pub frame_stack: Vec<Frame>,
	pub gc: Arc<RwLock<ObjectManager>>,
}

impl VmThread {
	pub fn new(vm: Arc<Vm>, loader: Option<LoaderRef>) -> Self {
		let loader = loader.unwrap_or(vm.loader.clone());
		Self {
			vm: vm.clone(),
			loader,
			frame_stack: Vec::new(),
			gc: vm.gc.clone(),
		}
	}

	/// Get or resolve a class, ensuring it and its dependencies are initialized.
	/// Follows JVM Spec 5.5 for recursive initialization handling.
	pub fn get_or_resolve_class(
		&self,
		what: &str,
		thread: Arc<VmThread>,
	) -> Result<Arc<RuntimeClass>, VmError> {
		// Phase 1: Load the class (short lock)
		let runtime_class = self
			.loader
			.lock()
			.unwrap()
			.get_or_load(what)
			.map_err(VmError::LoaderError)?;

		// Phase 2: Collect classes that need initialisation (short lock)
		let classes_to_init = {
			let mut loader = self.loader.lock().unwrap();
			let classes = loader.needs_init.clone();
			loader.needs_init.clear();
			classes
		};

		// Phase 3: Initialise each class (NO lock held - allows recursion)
		for class in classes_to_init {
			self.init(class, thread.clone())?;
		}

		Ok(runtime_class)
	}

	pub fn get_class(&self, what: &str) -> Result<Arc<RuntimeClass>, VmError> {
		self.loader
			.lock()
			.unwrap()
			.get_or_load(what)
			.map_err(VmError::LoaderError)
	}

	/// Initialize a class following JVM Spec 5.5.
	/// Handles recursive initialization by tracking which thread is initializing.
	fn init(&self, class: Arc<RuntimeClass>, thread: Arc<VmThread>) -> Result<(), VmError> {
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

		// Perform actual initialization
		trace!("Initializing class: {}", class.this_class);
		let result = (|| {
			// Initialize superclass first (if any)
			if let Some(ref super_class) = class.super_class {
				self.init(super_class.clone(), thread.clone())?;
			}

			// Run <clinit> if present
			let class_init_method = class.find_method("<clinit>", &MethodDescriptor::void());
			if let Ok(method) = class_init_method {
				Frame::new(
					method.code.clone().unwrap(),
					class.constant_pool.clone(),
					vec![],
					thread.clone(),
				)
				.execute()
				.map_err(|e| VmError::LoaderError(format!("Error in <clinit>: {:?}", e)))?;
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

	pub fn invoke_main(&self, what: &str, thread: Arc<VmThread>) {
		let method_ref = MethodRef {
			class: what.to_string(),
			name: "main".to_string(),
			desc: MethodDescriptor::psvm(),
		};

		self.invoke(method_ref, Vec::new(), thread)
			.expect("Main method died");
		return ();

		let class = self.get_or_resolve_class(what, thread.clone()).unwrap();
		println!("invoking main: {}", class.this_class);
		let main_method = class.find_method("main", &MethodDescriptor::psvm());
		println!("{:?}", main_method);
		if let Ok(meth) = main_method {
			let mut frame = Frame::new(
				meth.code.clone().unwrap(),
				class.constant_pool.clone(),
				vec![],
				thread.clone(),
			);
			// self.frame_stack.push(frame);
			frame.execute().expect("Error in main");
			// self.frame_stack.first().unwrap().execute();
		}
	}

	pub fn invoke(
		&self,
		method_reference: MethodRef,
		args: Vec<Value>,
		thread: Arc<VmThread>,
	) -> MethodCallResult {
		let class = self.get_or_resolve_class(&method_reference.class, thread.clone())?;
		let resolved_method = class
			.find_method(&method_reference.name, &method_reference.desc)
			.unwrap();
		trace!(
			"invoking '{}' from {}",
			method_reference.name,
			class.this_class
		);
		if resolved_method.flags.ACC_NATIVE {
			return self.invoke_native(&method_reference, args);
		}
		let mut frame = Frame::new(
			resolved_method.code.clone().unwrap(),
			class.constant_pool.clone(),
			args,
			thread.clone(),
		);
		frame.execute()
	}

	pub fn invoke_native(&self, method: &MethodRef, args: Vec<Value>) -> MethodCallResult {
		let symbol_name = generate_jni_method_name(method);
		trace!("searching for native symbol: {:?}", &symbol_name);

		let result = unsafe {
			// manually load relevant library for poc
			let lib = libloading::os::windows::Library::new(
				"C:\\Program Files\\Java\\jdk-25\\bin\\jvm_rs.dll",
			)
			.expect("load jvm_rs.dll");

			// build pointer to native fn
			let cp = CodePtr::from_ptr(
				lib.get::<*const ()>(symbol_name.as_ref())
					.unwrap()
					.as_raw_ptr(),
			);
			// build actual JNI interface that forms the table of
			// native functions that can be used to manipulate the JVM
			let jnienv = create_jni_function_table();

			// let args = build_args(args);

			// coerce my method descriptors into libffi C equivalents, then call
			// let l = method.build_cif().call::<jlong>(cp, args.as_ref());

			let mut storage = Vec::new();
			trace!("passing {args:?} to native fn");
			let built_args = build_args(args, &mut storage);
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
					// TODO: Convert jobject to Reference properly
					Ok(Some(Value::Reference(None)))
				}
			}
		};
		warn!("Invoke native not final");
		result
	}
}

fn build_args<'a>(params: Vec<Value>, storage: &'a mut Vec<Box<dyn Any>>) -> Vec<Arg<'a>> {
	// Store values in the provided storage
	storage.push(Box::new(create_jni_function_table()) as Box<dyn Any>);
	storage.push(Box::new(std::ptr::null_mut::<()>()) as Box<dyn Any>);

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
			Value::Reference(x) => storage.push(Box::new(x) as Box<dyn Any>),
		}
	}

	// Create args referencing the storage
	storage.iter().map(|boxed| arg(&**boxed)).collect()
}

pub fn generate_jni_method_name(method_ref: &MethodRef) -> String {
	let class_name = &method_ref.class.replace("/", "_");
	let method_name = &method_ref.name;
	format!("Java_{class_name}_{method_name}")
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
