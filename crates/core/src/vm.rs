#[cfg(libloading_docs)]
use super::os::unix as imp;
use std::ffi::c_void;
// the implementation used here doesn't matter particularly much...
#[cfg(all(not(libloading_docs), unix))]
use libloading::os::unix as imp;
#[cfg(all(not(libloading_docs), windows))]
use libloading::os::windows as imp;

use crate::class_file::{ClassFlags, MethodRef};
use crate::class_loader::ClassLoader;
use crate::objects::object_manager::ObjectManager;
use crate::thread::VmThread;
use crate::{MethodDescriptor, ThreadId};
use dashmap::DashMap;
use imp::Library;
use std::sync::{Arc, Mutex, RwLock};
use crate::objects::object::ReferenceKind;
use crate::error::VmError;

// struct AbstractObject<'a> {}
pub struct Vm {
	// Thread registry - maps ThreadId to VmThread
	pub threads: DashMap<ThreadId, Arc<VmThread>>,
	pub main_thread_id: ThreadId,
	pub loader: Arc<Mutex<ClassLoader>>,
	pub native_methods: DashMap<String, *const c_void>,
	pub native_libraries: DashMap<String, Library>,
	pub gc: Arc<RwLock<ObjectManager>>,
}

impl Vm {
	// start vm, loading main from classfile
	pub fn new() -> Arc<Self> {
		let vm = Arc::new(Self {
			threads: DashMap::new(),
			main_thread_id: ThreadId(0),
			loader: Arc::new(Mutex::from(ClassLoader::default())),
			native_methods: DashMap::new(),
			native_libraries: DashMap::new(),
			gc: Default::default(),
		});

		// Create main thread
		let thread = VmThread::new(vm.clone(), None);
		let thread_id = thread.id;

		// Store in VM
		vm.threads.insert(thread_id, thread.clone());

		// Set as current thread
		VmThread::set_current(thread_id);
		vm
	}

	pub fn load_native_library(&self, name: &str, lib: Library) {
		self.native_libraries.insert(name.to_string(), lib);
	}

	pub fn find_native_method(&self, name: &str) -> Option<*const c_void> {
		if name.contains("Java_jdk_internal_util_SystemProps$Raw_platformProperties") {
			println!("Pick me baws");
			let val = self.native_methods.iter().collect::<Vec<_>>();
			println!("{}", val.len());
		}

		if let Some(registered) = self.native_methods.get(name) {
			return Some(registered.clone());
		}

		{
			let lib = self.native_libraries.get("roast_vm.dll").unwrap();
			let res = unsafe { lib.get::<unsafe extern "system" fn()>(name.as_bytes()) };
			if res.is_ok() {
				let symbol = res.unwrap();
				let ptr = *symbol as *const c_void;
				self.native_methods.insert(name.to_string(), ptr);
				return Some(ptr);
			}
		}

		for entry in self.native_libraries.iter() {
			let (_lib_name, lib) = entry.pair();

			let res = unsafe { lib.get::<unsafe extern "system" fn()>(name.as_bytes()) };
			if res.is_ok() {
				let symbol = res.unwrap();
				let ptr = *symbol as *const c_void;
				self.native_methods.insert(name.to_string(), ptr);
				return Some(ptr);
			}
		}
		None
	}

	pub fn boot_strap(&self) -> Result<(), VmError> {
		let thread = self.threads.get(&self.main_thread_id).unwrap();
		let classes = vec![
			"java/lang/String",
			"java/lang/System",
			"java/lang/Class",
			"java/lang/ThreadGroup",
			"java/lang/Thread",
			"java/lang/Module",
			//unsafe internal?
			// "java/lang/reflect/Method",
			// "java/lang/ref/Finalizer",
			// "jdk/internal/misc/UnsafeConstants"
		];
		let _ = classes.iter().map(|e| thread.get_or_resolve_class(e));
		let prims = vec![
			("byte", "B"),
			("char", "C"),
			("double", "D"),
			("float", "F"),
			("int", "I"),
			("long", "J"),
			("short", "S"),
			("boolean", "Z")
		];
		let thread = self.threads.get(&self.main_thread_id).unwrap();

		for prim in prims {
			let klass =
				self.loader.lock().unwrap().primitive_class(prim.0);


			let class_class = thread.get_class("java/lang/Class")?;
			let name_obj = thread.intern_string(&prim.0);
			let flags = ClassFlags::from(1041u16);
			let class_obj = self.gc.write().unwrap().new_class(
				class_class.clone(),
				Some(ReferenceKind::ObjectReference(name_obj)),
				None,
				flags,
				true
			);
			klass.mirror.set(class_obj.lock().unwrap().id).unwrap();

			let prim_array_klass = self.loader.lock().unwrap().create_array_class(klass.clone());
			let arr_name_obj = thread.intern_string(&prim_array_klass.this_class);
			let arr_class_obj = self.gc.write().unwrap().new_class(
				class_class,
				Some(ReferenceKind::ObjectReference(arr_name_obj)),
				None,
				flags,
				false
			);
			prim_array_klass.mirror.set(arr_class_obj.lock().unwrap().id).unwrap();
		}

		let phase1ref = MethodRef {
			class: "java/lang/System".to_string(),
			name: "initPhase1".to_string(),
			desc: MethodDescriptor::void(),
		};
		thread.invoke(phase1ref, Vec::new())?;
		Ok(())
	}

	pub fn run(&self, what: &str) -> Result<(), VmError> {
		self.boot_strap()?;
		// Get main thread from DashMap
		let thread = self.threads.get(&self.main_thread_id).unwrap().clone();
		thread.invoke_main(what)
	}
}
