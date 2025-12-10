use crate::attributes::Attribute;
use crate::bimage::Bimage;
use crate::class::{InitState, RuntimeClass};
use crate::class_file::constant_pool::{ConstantPoolExt, ConstantPoolGet};
use crate::class_file::{ClassFile, ClassFlags, FieldData, FieldFlags, MethodData, MethodFlags};
use crate::{FieldType, MethodDescriptor};
use dashmap::DashMap;
use deku::DekuContainerRead;
use log::warn;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use crate::error::VmError;

pub type LoaderRef = Arc<Mutex<ClassLoader>>;

#[deprecated(note = "This method is deprecated and will be removed in future versions")]
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
			return Err(format!(
				"Could not find jmod/classes directory in module: {}",
				module
			));
		}
		classes_path
	} else {
		base.to_path_buf()
	};

	let class_path = path.join(format!("{}.class", fqn));
	if !class_path.exists() {
		return Err(format!(
			"Could not find class: {} in module: {}",
			fqn, module
		));
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
	classes: DashMap<(String, LoaderId), Arc<RuntimeClass>>,
	bimage: Bimage,
	pub needs_init: Vec<Arc<RuntimeClass>>,
}

type LoaderId = Option<u32>;

impl ClassLoader {
	pub fn new() -> Result<Self, String> {
		let loader = Self::default();
		// for entry in VM_CLASSES {
		//     let module_class = format!("{}/{}", "java.base", entry);;
		//     loader.load_class(&module_class)?;
		// }
		Ok(loader)
	}

	pub fn class_from_mirror_id(&self, id: u32) -> Option<Arc<RuntimeClass>> {
		self.classes
			.iter()
			.map(|x| x.value().clone())
			.find(|e| *e.mirror.wait() == id)
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
	pub fn get_or_load(
		&mut self,
		class_name: &str,
		loader: LoaderId,
	) -> Result<Arc<RuntimeClass>, VmError> {
		if let Some(class) = self.classes.get(&(class_name.to_string(), loader)) {
			return Ok(class.clone());
		}
		if class_name.starts_with("[") {
			let component_name = class_name.strip_prefix("[").unwrap();
			let component = match component_name.chars().next() {
				Some('B') => self.get_or_load("byte", None),
				Some('C') => self.get_or_load("char", None),
				Some('D') => self.get_or_load("double", None),
				Some('F') => self.get_or_load("float", None),
				Some('I') => self.get_or_load("int", None),
				Some('J') => self.get_or_load("long", None),
				Some('S') => self.get_or_load("short", None),
				Some('Z') => self.get_or_load("boolean", None),
				Some('[') => self.get_or_load(component_name, None),
				Some('L') => {
					let class_name = component_name
						.strip_prefix('L')
						.and_then(|s| s.strip_suffix(';'))
						.expect("invalid L descriptor");
					self.get_or_load(class_name, None)
				}
				None => Err(VmError::LoaderError("empty component descriptor".to_string())),
				_ => Err(VmError::LoaderError(format!("invalid component descriptor: {}", component_name))),
			}?;
			// let component = self.get_or_load(component_name, None)?;
			let arr_class = self.create_array_class(
				component,
			);
			self.needs_init.push(arr_class.clone());
			return Ok(arr_class)
		}
		let class = self.load_class(class_name, loader)?;
		self.needs_init.push(class.clone());
		Ok(class)
	}

	/* pub fn classes(&self) -> HashMap<String, ClassFile> {
		self.classes.clone()
	}*/

	fn load_class(&mut self, what: &str, loader: LoaderId) -> Result<Arc<RuntimeClass>, VmError> {
		let (module, class_fqn) = ("", what);
		let bytes = self
			.bimage
			.get_class(module, class_fqn).or_else(|e| Self::load_class_from_disk(what)).map_err(|e1| {
			let classes = self.classes.iter().map(|x| {
				x.this_class.clone()
			}).collect::<Vec<_>>();
			// println!("{:#?}", classes);
			VmError::LoaderError(format!("failed to find class: {:?}\n{:#?}", what, classes))
		})?;
		let (_, cf) = ClassFile::from_bytes((bytes.as_ref(), 0))
			.map_err(|e| VmError::DekuError(e))?;
		let runtime = self.runtime_class(cf);
		let arced = Arc::new(runtime);
		let option = self
			.classes
			.insert((class_fqn.to_string(), loader), arced.clone());
		if option.is_some() {
			warn!("Replaced loaded class: {}", class_fqn)
		}
		Ok(arced)
	}

	fn load_class_from_disk(what: &str) -> Result<Vec<u8>, String> {
		let class_path = std::env::args()
			.nth(1)
			.unwrap_or("./data".to_string())
			.replace("\\", "/");

		let path = format!("{class_path}/{what}.class");
		log::info!("Loading class from path: {}", path);
		let mut class_file = File::open(path).map_err(|e| {e.to_string()})?;
		let mut bytes = Vec::new();
		class_file.read_to_end(&mut bytes).unwrap();
		Ok(bytes)
	}

	fn runtime_class(&mut self, class_file: ClassFile) -> RuntimeClass {
		let constant_pool = class_file.constant_pool.clone();
		let access_flags = ClassFlags::from(class_file.access_flags);
		let this_class = {
			let cl = class_file
				.constant_pool
				.get_class_info(class_file.this_class)
				.unwrap();
			let name = class_file.constant_pool.get_string(cl.name_index).unwrap();
			name
		};
		let super_class = {
			if this_class.eq("java/lang/Object") {
				debug_assert_eq!(this_class, "java/lang/Object");
				debug_assert_eq!(class_file.super_class, 0u16);
				None
			} else {
				debug_assert_ne!(class_file.super_class, 0u16);
				let super_info = constant_pool
					.get_class_info(class_file.super_class)
					.unwrap();
				let name = constant_pool.get_string(**super_info).unwrap();
				Some(self.get_or_load(&*name, None).unwrap())
			}
		};

		if let Some(super_cl) = super_class.clone() {
			let super_is_object = super_cl.super_class.is_none();

			if access_flags.INTERFACE {
				debug_assert!(super_is_object);
			}
		}

		let interfaces = class_file
			.interfaces
			.iter()
			.copied()
			.map(|e| {
				let interface_info = constant_pool.get_class_info(e).unwrap();
				let name = constant_pool.get_string(interface_info.name_index).unwrap();
				self.get_or_load(&name, None).unwrap()
			})
			.collect::<Vec<_>>();

		let fields = class_file
			.fields
			.iter()
			.map(|e| {
				let name = constant_pool.get_string(e.name_index).unwrap();
				let flags = FieldFlags::from(e.access_flags);
				let desc = constant_pool
					.get_string(e.descriptor_index)
					.map(|e| FieldType::parse(&e))
					.unwrap()
					.unwrap();
				let value = e.attributes.first().and_then(|x| {
					if let Attribute::ConstantValue(val) =
						constant_pool.parse_attribute(x.clone()).unwrap()
					{
						Some(val.into())
					} else {
						None
					}
				});
				let value = Arc::new(Mutex::new(value));
				FieldData {
					name,
					flags,
					desc,
					value,
				}
			})
			.collect::<Vec<_>>();

		let methods = class_file
			.methods
			.iter()
			.map(|e| {
				let name = constant_pool.get_string(e.name_index).unwrap();
				let flags = MethodFlags::from(e.access_flags);
				let desc = constant_pool
					.get_string(e.descriptor_index)
					.map(|e| MethodDescriptor::parse(&e))
					.unwrap()
					.unwrap();
				let code = e.attributes.first().and_then(|x| {
					if let Attribute::Code(val) = constant_pool.parse_attribute(x.clone()).unwrap()
					{
						Some(val)
					} else {
						None
					}
				});
				let line_number_table = code.as_ref().and_then(|x| {
					x.attributes.iter().find_map(|attr| {
						match constant_pool.parse_attribute(attr.clone()).ok()? {
							Attribute::LineNumberTable(table) => Some(table.line_number_table),
							_ => None,
						}
					})
				});

				MethodData {
					name,
					flags,
					desc,
					code,
					line_number_table,
				}
			})
			.collect::<Vec<_>>();

		let super_classes = std::iter::successors(super_class.clone(), |sc| sc.super_class.clone())
			.collect::<Vec<_>>();

		#[allow(clippy::mutable_key_type)]
		let mut result = HashSet::new();
		let mut stack = super_classes.clone();
		stack.extend(interfaces.iter().cloned());

		while let Some(c) = stack.pop() {
			if result.insert(c.clone()) {
				if let Some(sc) = &c.super_class {
					stack.push(sc.clone());
				}
				stack.extend(c.interfaces.iter().cloned());
			}
		}
		let super_interfaces = result
			.iter()
			.cloned()
			.filter(|e| e.access_flags.INTERFACE)
			.collect::<Vec<_>>();

		let source_file = class_file.attributes.iter().find_map(|attr| {
			match constant_pool.parse_attribute(attr.clone()).ok()? {
				Attribute::SourceFile(index) => {
					constant_pool.get_string(index).ok()
				},
				_ => None,
			}
		});

		RuntimeClass {
			constant_pool,
			access_flags,
			this_class,
			super_class,
			interfaces,
			fields,
			methods,
			init_state: Mutex::new(InitState::NotInitialized),
			mirror: OnceLock::new(),
			super_classes,
			super_interfaces,
			component_type: None,
			source_file,
		}
	}
	// pub fn get_or_create_array_class(class_name: &str) -> RuntimeClass {
	// 	if let Some(class) = self.classes.get(&(class_name.to_string(), loader)) {
	// 		return Ok(class.clone());
	// 	}
	// 	let class = self.load_class(class_name, loader)?;
	// 	self.needs_init.push(class.clone());
	// 	Ok(class)
	// }

	pub fn create_array_class(
		&mut self,
		component: Arc<RuntimeClass>,
	) -> Arc<RuntimeClass> {
		// let name = format!("[{}", component.descriptor()); // e.g., "[Ljava/lang/String;"
		let object_class: Arc<RuntimeClass> = self.get_or_load("java/lang/Object", None).unwrap();
		let cloneable: Arc<RuntimeClass> = self.get_or_load("java/lang/Cloneable", None).unwrap();
		let serializable: Arc<RuntimeClass> = self.get_or_load("java/io/Serializable", None).unwrap();
		let name = match component.this_class.as_str() {
			"byte" => "[B".to_string(),
			"char" => "[C".to_string(),
			"double" => "[D".to_string(),
			"float" => "[F".to_string(),
			"int" => "[I".to_string(),
			"long" => "[J".to_string(),
			"short" => "[S".to_string(),
			"boolean" => "[Z".to_string(),
			s if s.starts_with('[') => format!("[{}", s),      // nested array
			s => format!("[L{};", s),                          // object class
		};
		let klass =
			RuntimeClass {
				constant_pool: Arc::new(vec![]),
				access_flags: ClassFlags {
					MODULE: false,
					ENUM: false,
					ANNOTATION: false,
					SYNTHETIC: false,
					ABSTRACT: true,
					INTERFACE: false,
					SUPER: false,
					FINAL: true,
					PUBLIC: true,
				},
				this_class: name.clone(),
				super_class: Some(object_class.clone()),
				interfaces: vec![cloneable.clone(), serializable.clone()],
				fields: vec![],
				methods: vec![],  // clone() is handled specially
				mirror: OnceLock::new(),
				init_state: Mutex::new(InitState::NotInitialized), // arrays need no <clinit>
				super_classes: vec![object_class],
				super_interfaces: vec![cloneable, serializable],
				component_type: Some(component),  // new field
				source_file: None,
			};
		let klass = Arc::new(klass);
		self.classes.insert((name.to_string(), None), klass.clone());
		klass
	}

	pub fn primitive_class(&mut self, name: &str) -> Arc<RuntimeClass> {
		
		
		

		

		
		
		let klass = Arc::new(RuntimeClass {
			constant_pool: Arc::new(vec![]),
			access_flags: ClassFlags {
				MODULE: false,
				ENUM: false,
				ANNOTATION: false,
				SYNTHETIC: false,
				ABSTRACT: false,
				INTERFACE: false,
				SUPER: false,
				FINAL: false,
				PUBLIC: false,
			},
			this_class: name.to_string(),
			super_class: None,
			interfaces: vec![],
			fields: vec![],
			methods: vec![],
			mirror: Default::default(),
			init_state: Mutex::new(InitState::NotInitialized),
			super_classes: vec![],
			super_interfaces: vec![],
			component_type: None,
			source_file: None,
		});

		self.classes.insert((name.to_string(), None), klass.clone());
		klass
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
