use crate::class_file::ClassFile;
use crate::class_loader::ClassLoader;
use crate::object::Object;
use crate::object_manager::ObjectManager;
use crate::thread::VmThread;
use crate::Frame;
use libloading::os::windows::Symbol;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

// struct AbstractObject<'a> {}
pub struct Vm {
	// for now, model just a single thread
	pub thread: Mutex<Vec<Arc<VmThread>>>,
	pub loader: Arc<Mutex<ClassLoader>>,
	pub native_methods: HashMap<String, Symbol<()>>,
	pub gc: Arc<RwLock<ObjectManager>>,
}

impl Vm {
	// start vm, loading main from classfile
	pub fn new(what: &str) -> Arc<Self> {
		let vm = Arc::new(Self {
			loader: Arc::new(Mutex::from(ClassLoader::default())),
			thread: Mutex::new(Vec::new()),
			native_methods: Default::default(),
			gc: Default::default(),
		});
		let thread = Arc::new(VmThread::new(vm.clone(), None));
		vm.thread.lock().unwrap().push(thread.clone());
		thread.invoke_main(what, thread.clone());
		vm.clone()
	}
}
