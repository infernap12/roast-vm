use std::sync::{Arc, Mutex};
use crate::class_file::ClassFile;
use crate::class_loader::{ClassLoader, LoaderRef};
use crate::Frame;
use crate::vm::Vm;

// A thread of execution
pub struct VmThread {
	vm: Arc<Vm>,
	loader: Arc<Mutex<ClassLoader>>,
	frame_stack: Vec<Frame>
}

impl VmThread {
	
	pub fn new(vm: Arc<Vm>, loader: LoaderRef, ) -> Self {
		Self {
			vm,
			loader,
			frame_stack: Vec::new(),
		}
	}
	
	

	pub fn get_or_resolve_class(&self, what: &str) -> () {
		let class_file = self.loader.lock().unwrap().get_or_load(what).unwrap();
		self.init(class_file)
	}
	
	fn init(&self, class: Arc<ClassFile>) {
		class.methods.first()
	}
}