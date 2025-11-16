use std::sync::{Arc, Mutex};
use crate::class_file::ClassFile;
use crate::class_loader::ClassLoader;
use crate::Frame;
use crate::thread::VmThread;

// struct AbstractObject<'a> {}
pub struct Vm {
	// for now, model just a single thread
	pub thread: Mutex<Vec<Arc<VmThread>>>,
	pub loader: Arc<Mutex<ClassLoader>>
}

impl Vm {
	// start vm, loading main from classfile
	pub fn new(what: &str) -> Arc<Self> {
		let vm = Arc::new(Self {
			loader: Arc::new(Mutex::from(ClassLoader::default())),
			thread: Mutex::new(Vec::new()),
		});
		let thread = Arc::new(VmThread::new(vm.clone(), None));
		vm.thread.lock().unwrap().push(thread.clone());
		thread.invoke_main(what, thread.clone());
		vm.clone()
	}
}