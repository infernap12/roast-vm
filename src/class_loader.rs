use std::collections::hash_map::{Entry, Iter};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use log::warn;
use crate::bimage::Bimage;
use crate::class::RuntimeClass;
use crate::class_file::{ClassFile, ClassFlags, ConstantClassInfo, ConstantPoolExt, CpInfo};


pub type LoaderRef = Arc<Mutex<ClassLoader>>;

#[deprecated(
	note = "This method is deprecated and will be removed in future versions"
)]
pub fn resolve_path(what: &str) -> Result<(PathBuf, String), String> {
	let (module, fqn) = what.split_once("/").unwrap_or(("", what));
	let module = dot_to_path(module);
	let fqn = dot_to_path(fqn);

	let base = Path::new("./data");

	if !base.exists() {
		return Err("Could not find data directory!".to_string());
	}
	let path = if !module.is_empty() {
		let module_path = base.join(&module);
		if !module_path.exists() {
			return Err(format!("Could not find module: {}", module));
		}

		let classes_path = module_path.join("jmod/classes");
		if !classes_path.exists() {
			return Err(format!("Could not find jmod/classes directory in module: {}", module));
		}
		classes_path
	} else { base.to_path_buf() };


	let class_path = path.join(format!("{}.class", fqn));
	if !class_path.exists() {
		return Err(format!("Could not find class: {} in module: {}", fqn, module));
	}

	Ok((class_path, path_to_dot(&fqn)))
}

/// A struct representing a ClassLoader which is responsible for managing and storing classes and their associated binary image information.
///
/// # Fields
///
/// - `classes`: A `HashMap<String, ClassFile>` that maps class names (as `String`) to their corresponding `ClassFile` instances.
///   This serves as the storage for loaded classes.
///
/// - `bimage`: A `Bimage` instance used for handling binary image data associated with the class loading process.
///
/// # Derives
///
/// - `Default`: The `ClassLoader` can be instantiated with default values using the `Default` trait.
///   By default, `classes` will be an empty `HashMap` and `bimage` will use its own default initialization.
///
/// # Usage
///
/// The `ClassLoader` struct is useful in environments or applications that require dynamic
/// or on-demand loading of classes with related binary image data.
///
/// Example:
/// ```
/// use std::collections::HashMap;
/// let class_loader = ClassLoader::default();
/// assert!(class_loader.classes.is_empty());
/// ```
#[derive(Default)]
pub struct ClassLoader {
	classes: HashMap<String, Arc<ClassFile>>,
	bimage: Bimage,
}

impl ClassLoader {
	pub fn new() -> Result<Self, String> {
		let mut loader = Self::default();
		// for entry in VM_CLASSES {
		//     let module_class = format!("{}/{}", "java.base", entry);;
		//     loader.load_class(&module_class)?;
		// }
		Ok(loader)
	}

	/// Retrieves an `Arc<ClassFile>` from the internal storage, or attempts to load it if not already present.
	///
	/// # Arguments
	/// * `what` - A fully qualified class name (e.g. "java.base/java.lang.Object" or "java.lang.String")
	///
	/// # Returns
	/// * `Ok(Arc<ClassFile>)` - A thread-safe reference-counted pointer to the requested `ClassFile` on success, 
	///   either retrieved from the storage or successfully loaded.
	/// * `Err(String)` - An error message string if the class could not be loaded due to some failure.
	///
	/// # Behavior
	/// * Checks if the requested class is already present in the internal `classes` storage.
	/// * If the class is not present, it attempts to load the class by calling `self.load_class()`.
	/// * If the class loading fails, the function returns an error.
	/// * If successful, it retrieves the `Arc<ClassFile>` from the storage and returns a clone of it.
	/// * Using `Arc` allows the ClassFile to be shared between multiple threads safely.
	///
	/// # Example
	/// ```rust
	/// # use std::sync::Arc;
	/// let mut class_loader = ClassLoader::new()?;
	/// match class_loader.get_or_load("java.base/java.lang.Object") {
	///     Ok(class_file) => {
	///         // class_file is an Arc<ClassFile> that can be cloned and shared
	///         let shared_class = class_file.clone();
	///     }
	///     Err(e) => {
	///         eprintln!("Failed to load class: {}", e);
	///     }
	/// }
	/// ```
	pub fn get_or_load(&mut self, what: &str) -> Result<Arc<ClassFile>, String> {
		if let Some(class) = self.classes.get(what) {
			return Ok(class.clone());
		}
		let class = self.load_class(what)?;
		self.classes.insert(what.to_string(), class.clone());
		Ok(class)
	}

	/* pub fn classes(&self) -> HashMap<String, ClassFile> {
		 self.classes.clone()
	 }*/

	fn load_class(&mut self, what: &str) -> Result<Arc<ClassFile>, String> {
		let (module, class_fqn) = what.split_once("/").unwrap_or(("", what));
		let bytes = self.bimage.get_class(module, class_fqn);
		let (_, cf) = ClassFile::from_bytes_interpreted((bytes.as_ref(), 0))
			.map_err(|e| format!("failed to parse class file: {}", e))?;
		let arced = Arc::new(cf);
		let option = self.classes.insert(class_fqn.to_string(), arced.clone());
		if option.is_some() { warn!("Replaced loaded class: {}", class_fqn) }
		Ok(arced)
	}





	fn runtime_class(&self, class_file: ClassFile) -> RuntimeClass {
		let constant_pool = class_file.constant_pool.clone();
		let access_flags = ClassFlags::from(class_file.access_flags);
		let this_class = {
			let this_class_info = class_file.constant_pool.get_constant(class_file.this_class)
			if let Some(CpInfo::Class(class_info)) = this_class_info {
				class_file.constant_pool.get_string(class_info.name_index)
			}



		}








		RuntimeClass {
			constant_pool,
			access_flags,
			this_class: "".to_string(),
			super_class: Arc::new(RuntimeClass {}),
			interfaces: vec![],
			fields: vec![],
			methods: vec![],
		}
	}
}

fn dot_to_path(s: &str) -> String {
	s.replace(".", "/")
}

pub fn path_to_dot(s: &str) -> String {
	s.replace("/", ".")
}

const VM_CLASSES: &[&str] = &[
	"java.lang.Object",
	"java.lang.String",
	"java.lang.Class",
	"java.lang.Cloneable",
	"java.lang.ClassLoader",
	"java.io.Serializable",
	"java.lang.System",
	"java.lang.Throwable",
	"java.lang.Error",
	"java.lang.Exception",
	// "java.lang.RuntimeException",
	// "java.security.ProtectionDomain",
	// "java.security.SecureClassLoader",
	// "java.lang.ClassNotFoundException",
	// "java.lang.Record",
	// "java.lang.NoClassDefFoundError",
	// "java.lang.LinkageError",
	// "java.lang.ClassCastException",
	// "java.lang.ArrayStoreException",
	// "java.lang.VirtualMachineError",
	// "java.lang.InternalError",
	// "java.lang.OutOfMemoryError",
	// "java.lang.StackOverflowError",
	// "java.lang.IllegalMonitorStateException",
	// "java.lang.ref.Reference",
	// "java.lang.IllegalCallerException",
	// "java.lang.ref.SoftReference",
	// "java.lang.ref.WeakReference",
	// "java.lang.ref.FinalReference",
	// "java.lang.ref.PhantomReference",
	// "java.lang.ref.Finalizer",
	// "java.lang.Thread",
	// "java.lang.Thread.FieldHolder",
	// "java.lang.Thread.Constants",
	// "java.lang.ThreadGroup",
	// "java.lang.BaseVirtualThread",
	// "java.lang.VirtualThread",
	// "java.lang.BoundVirtualThread",
	// "java.util.Properties",
	// "java.lang.Module",
	// "java.lang.reflect.AccessibleObject",
	// "java.lang.reflect.Field",
	// "java.lang.reflect.Parameter",
	// "java.lang.reflect.Method",
	// "java.lang.reflect.Constructor",
	// "java.lang.Runnable",
	// "jdk.internal.vm.ContinuationScope",
	// "jdk.internal.vm.Continuation",
	// "jdk.internal.vm.StackChunk",
	// "reflect.MethodAccessorImpl",
	// "reflect.ConstantPool",
	// "reflect.CallerSensitive",
	// "reflect.DirectConstructorHandleAccessor.NativeAccessor",
	// "java.lang.invoke.DirectMethodHandle",
	// "java.lang.invoke.MethodHandle",
	// "java.lang.invoke.VarHandle",
	// "java.lang.invoke.MemberName",
	// "java.lang.invoke.ResolvedMethodName",
	// "java.lang.invoke.MethodHandleImpl",
	// "java.lang.invoke.MethodHandleNatives",
	// "java.lang.invoke.LambdaForm",
	// "java.lang.invoke.MethodType",
	// "java.lang.BootstrapMethodError",
	// "java.lang.invoke.CallSite",
	// "jdk.internal.foreign.abi.NativeEntryPoint",
	// "jdk.internal.foreign.abi.ABIDescriptor",
	// "jdk.internal.foreign.abi.VMStorage",
	// "jdk.internal.foreign.abi.CallConv",
	// "java.lang.invoke.ConstantCallSite",
	// "java.lang.invoke.MutableCallSite",
	// "java.lang.invoke.VolatileCallSite",
	// "java.lang.AssertionStatusDirectives",
	// "java.lang.StringBuffer",
	// "java.lang.StringBuilder",
	// "jdk.internal.misc.UnsafeConstants",
	// "jdk.internal.misc.Unsafe",
	// "jdk.internal.module.Modules",
	// "java.io.ByteArrayInputStream",
	// "java.net.URL",
	// "java.lang.Enum",
	// "java.util.jar.Manifest",
	// "jdk.internal.loader.BuiltinClassLoader",
	// "jdk.internal.loader.ClassLoaders",
	// "jdk.internal.loader.ClassLoaders.AppClassLoader",
	// "jdk.internal.loader.ClassLoaders.PlatformClassLoader",
	// "java.security.CodeSource",
	// "java.util.concurrent.ConcurrentHashMap",
	// "java.util.ArrayList",
	// "java.lang.StackTraceElement",
	// "java.nio.Buffer",
	// "java.lang.StackWalker",
	// "java.lang.StackStreamFactory.AbstractStackWalker",
	// "java.lang.ClassFrameInfo",
	// "java.lang.StackFrameInfo",
	// "java.lang.LiveStackFrameInfo",
	// "java.util.concurrent.locks.AbstractOwnableSynchronizer",
	// "java.lang.Boolean",
	// "java.lang.Character",
	// "java.lang.Float",
	// "java.lang.Double",
	// "java.lang.Byte",
	// "java.lang.Short",
	// "java.lang.Integer",
	// "java.lang.Long",
	// "java.lang.Void",
	// "java.util.Iterator",
	// "java.lang.reflect.RecordComponent",
	// "jdk.internal.vm.vector.VectorSupport",
	// "jdk.internal.vm.vector.VectorPayload",
	// "jdk.internal.vm.vector.Vector",
	// "jdk.internal.vm.vector.VectorMask",
	// "jdk.internal.vm.vector.VectorShuffle",
	// "jdk.internal.vm.FillerObject",
];

// fn load_class(module: &str, fqn: &str) -> Result<ClassFile, String> {
//     let path = resolve_path(module, fqn)
//         .map_err(|e| format!("failed to resolve class file location: {}", e))?;
//     let mut class_file = File::open(path)
//         .map_err(|e| format!("failed to open class file: {}", e))?;
//     let mut bytes = Vec::new();
//     class_file.read_to_end(&mut bytes)
//         .map_err(|e| format!("failed to read class file: {}", e))?;
//     let (_, cf) = ClassFile::from_bytes_interpreted((bytes.as_ref(), 0))
//         .map_err(|e| format!("failed to parse class file: {}", e))?;
//     Ok(cf)
// }