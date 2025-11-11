use std::sync::{Arc, Mutex};
use crate::class_loader::ClassLoader;
use crate::Frame;
use crate::thread::VmThread;

// struct AbstractObject<'a> {}
pub struct Vm {
	// for now, model just a single thread
	pub thread: Vec<VmThread>,
	pub loader: Arc<Mutex<ClassLoader>>
}

impl Vm {
	pub fn new() -> Self {
		Self {
			loader: Arc::new(Mutex::from(ClassLoader::default())),
			thread: Vec::new(),
		}
	}

	pub fn get_or_resolve_class(&self, what: &str) -> () {
		let class_file = self.loader.lock().unwrap().get_or_load(what).unwrap();
		self.init(class_file)
	}

	fn init() -> () {}
}