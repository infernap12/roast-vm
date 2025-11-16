use crate::class::RuntimeClass;
use crate::class_file::{ClassFile, MethodRef};
use crate::class_loader::{ClassLoader, LoaderRef};
use crate::vm::Vm;
use crate::{Frame, MethodDescriptor, Value, VmError};
use deku::DekuError::Incomplete;
use std::sync::{Arc, Mutex};

type MethodCallResult = Result<Option<Value>, VmError>;

// A thread of execution
pub struct VmThread {
	pub vm: Arc<Vm>,
	pub loader: Arc<Mutex<ClassLoader>>,
	pub frame_stack: Vec<Frame>,
}

impl VmThread {
	pub fn new(vm: Arc<Vm>, loader: Option<LoaderRef>) -> Self {
		let loader = loader.unwrap_or(vm.loader.clone());
		Self {
			vm,
			loader,
			frame_stack: Vec::new(),
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
			.map_err(|e| VmError::LoaderError(e))?;

		// Phase 2: Collect classes that need initialization (short lock)
		let classes_to_init = {
			let mut loader = self.loader.lock().unwrap();
			let classes = loader.needs_init.clone();
			loader.needs_init.clear();
			classes
		};

		// Phase 3: Initialize each class (NO lock held - allows recursion)
		for class in classes_to_init {
			self.init(class, thread.clone())?;
		}

		Ok(runtime_class)
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
					println!(
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
		println!("Initializing class: {}", class.this_class);
		let result = (|| {
			// Initialize superclass first (if any)
			if let Some(ref super_class) = class.super_class {
				self.init(super_class.clone(), thread.clone())?;
			}

			// Run <clinit> if present
			let class_init_method = class.find_method("<clinit>", MethodDescriptor::void());
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
					println!("Class {} initialized successfully", class.this_class);
				}
				Err(ref e) => {
					*state = InitState::Error(format!("{:?}", e));
				}
			}
		}

		result
	}

	pub fn invoke_main(&self, what: &str, thread: Arc<VmThread>) {
		let class = self.get_or_resolve_class(what, thread.clone()).unwrap();
		println!("invoking main: {}", class.this_class);
		let main_method = class.find_method("main", MethodDescriptor::psvm());
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

	pub fn invoke(&self, method_reference: MethodRef, thread: Arc<VmThread>) -> MethodCallResult {
		let class = self.get_or_resolve_class(&method_reference.class, thread.clone())?;
		let resolved_method = class
			.find_method(&method_reference.name, method_reference.desc)
			.unwrap();
		if resolved_method.flags.ACC_NATIVE {
			return self.invoke_native();
		}
		let mut frame = Frame::new(
			resolved_method.code.clone().unwrap(),
			class.constant_pool.clone(),
			vec![],
			thread.clone(),
		);
		frame.execute()
	}

	pub fn invoke_native(&self) -> MethodCallResult {
		todo!("Invoke native")
	}
}
