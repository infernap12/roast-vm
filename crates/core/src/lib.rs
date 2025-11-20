//! A simple JVM implementation in Rust that aims to run Java class files.
//!
//! # Overview
//! This crate provides functionality to:
//! - Load and parse Java class files
//! - Execute JVM bytecode instructions
//! - Manage class loading and object creation
//! - Handle method invocation and stack frames
//!
//! # Core Types
//! - [`Frame`] - Represents a JVM stack frame for method execution
//! - [`Value`] - Represents JVM runtime values/primitives

//! - [`BaseType`] - JVM primitive types
//! - [`MethodDescriptor`] - Method signature information
//! - [`FieldType`] - Field type information

use crate::attributes::{Attribute, CodeAttribute, Ops};
use crate::class_file::constant_pool::ConstantPoolExt;
use crate::class_file::constant_pool::{ConstantPoolError, ConstantPoolGet};
use crate::class_file::{Bytecode, ClassFile, ConstantPoolEntry, MethodData};
use crate::object::Object;
use crate::thread::VmThread;
use crate::Value::Reference;
use deku::{DekuContainerRead, DekuError};
use deku_derive::{DekuRead, DekuWrite};
use env_logger::Builder;
use log::{warn, LevelFilter};
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use vm::Vm;

mod attributes;
mod bimage;
mod class;
mod class_file;
mod class_loader;
mod jni;
mod macros;
mod native_libraries;
mod object;
mod rng;
mod thread;
mod vm;

const NULL: Value = Value::Reference(None);

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
/// pseudo main
pub fn run() {
	Builder::from_default_env()
		.filter_level(LevelFilter::Trace)
		.filter_module("deku", LevelFilter::Warn)
		.filter_module("jvm_rs_core::class_file::class_file", LevelFilter::Info)
		.filter_module("jvm_rs_core::attributes", LevelFilter::Info)
		.init();
	// let mut cl = ClassLoader::new().unwrap();
	// cl.load_class("org.example.App").expect("TODO: panic message");
	// let clazz = cl.get_or_load("org.example.App").unwrap();
	// for (i, (k, v)) in cl.classes().iter().enumerate() {
	//     std::fs::write(format!("./output/{}-{}.txt", i, class_loader::path_to_dot(k)), format!("{}\n{}", k, v)).unwrap();
	// }

	/*let mut class_file = File::open("./data/org/example/Main.class").unwrap();
	let mut bytes = Vec::new();
	class_file.read_to_end(&mut bytes).unwrap();
	let (_rest, clazz) = ClassFile::from_bytes((bytes.as_ref(), 0)).unwrap();
	let method = clazz.methods.get(1).unwrap().clone();
	let code = method
		.attributes
		.iter()
		.find_map(|x| {
			if let Some(Attribute::Code(code_attr)) = &x.get(&clazz) {
				Some(code_attr.clone())
			} else {
				None
			}
		})
		.unwrap();
	// let frame = Frame::new();
	// println!("{}", code);
	let mut buf = Vec::new();
	let bytes = code.code_length.to_be_bytes();
	buf.extend_from_slice(&bytes);
	buf.extend_from_slice(&code.code.clone());
	let (_rest, ops) = Bytecode::from_bytes((buf.as_ref(), 0)).unwrap();
	let var_table = code
		.attributes
		.iter()
		.find_map(|x| {
			if let Some(Attribute::LocalVariableTable(varTableAttr)) = &x.get(&clazz) {
				Some(varTableAttr.clone())
			} else {
				None
			}
		})
		.unwrap();
	println!("{}", clazz);*/
	// let pool = clazz.constant_pool;
	let mut vm = Vm::new("org/example/Main");

	// println!("{:?}", ops);
	// println!("{:?}", var_table.local_variable_table);
	// vm.method(ops.clone(), code, var_table);
}

/// A reference-counted, thread-safe pointer to an Object.
type ObjectRef = Arc<Mutex<Object>>;

/// Represents a JVM runtime value.
///
/// This enum covers all primitive types and object references that can exist
/// on the operand stack or in local variables during bytecode execution.
#[derive(Debug, Clone)]
enum Value {
	/// Boolean value (true/false)
	Boolean(bool),
	/// Unicode character
	Char(char),
	/// 32-bit floating point
	Float(f32),
	/// 64-bit floating point
	Double(f64),
	/// Signed 8-bit integer
	Byte(i8),
	/// Signed 16-bit integer
	Short(i16),
	/// Signed 32-bit integer
	Int(i32),
	/// Signed 64-bit integer
	Long(i64),
	/// Reference to an object (or null)
	Reference(Option<ObjectRef>),
}

/// Represents a JVM stack frame for method execution.
///
/// A frame contains all the execution state needed to run a single method:
/// - Program counter (PC) tracking the current bytecode instruction
/// - Operand stack for intermediate values during computation
/// - Local variables for method parameters and local vars
/// - Reference to the constant pool for the class
/// - The bytecode to execute
/// - Reference to the thread executing this frame
struct Frame {
	/// Program counter - index of the current bytecode instruction
	pc: u16,
	/// Operand stack for intermediate values
	stack: Vec<Value>,
	/// Local variables (includes method parameters)
	vars: Vec<Value>,
	/// Constant pool from the class file
	pool: Arc<Vec<ConstantPoolEntry>>,

	/// The bytecode instructions for this method
	bytecode: Bytecode,

	/// The thread executing this frame
	thread: Arc<VmThread>,
}

impl Display for Frame {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"PC: {}\nStack: {:?}\nVars: {:?}",
			self.pc, self.stack, self.vars
		)
	}
}

//  println!("State:\n\tStack: {:?}\n\tLocals :{:?}\n", self.stack, self.vars) }

impl Frame {
	fn load_constant(index: u8) {}
	fn new(
		code_attr: CodeAttribute,
		pool: Arc<Vec<ConstantPoolEntry>>,
		mut locals: Vec<Value>,
		thread: Arc<VmThread>,
	) -> Self {
		let max_stack = code_attr.max_stack as usize;
		let max_local = code_attr.max_locals as usize;
		let bytes = code_attr.code_length.to_be_bytes();
		let mut buf = Vec::new();
		buf.extend_from_slice(&bytes);
		buf.extend_from_slice(&code_attr.code.clone());
		let (_rest, bytecode) = Bytecode::from_bytes((buf.as_ref(), 0)).unwrap();
		let extend = vec![Value::Reference(None); max_local - locals.len()];
		locals.extend_from_slice(&extend);
		Frame {
			pc: 0,
			stack: Vec::with_capacity(max_stack),
			vars: locals,
			pool,
			bytecode,
			thread,
		}
	}
	fn execute(&mut self) -> Result<Option<Value>, VmError> {
		let binding = self.bytecode.code.clone();
		let mut ops = binding.iter();
		for op in ops {
			println!("Executing Op: {:?}", op);
			let result = self.execute_instruction(op);
			match result {
				Ok(ExecutionResult::Return(c)) => return Ok(None),
				Ok(ExecutionResult::ReturnValue(val)) => return Ok(Some(val)),
				Ok(_) => {
					println!(
						"State:\n\tStack: {:?}\n\tLocals :{:?}\n",
						self.stack, self.vars
					)
				}
				Err(_) => {
					panic!("Mission failed, we'll get em next time")
				}
			}
		}
		Err(VmError::ExecutionError)
	}
}

/// Represents JVM primitive types used in field and method descriptors.
///
/// Each variant corresponds to a single-character type code used in the JVM:
/// - B: byte
/// - C: char
/// - D: double
/// - F: float
/// - I: int
/// - J: long
/// - S: short
/// - Z: boolean
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
#[deku(seek_from_current = "-1")]
#[derive(Clone)]
pub enum BaseType {
	/// B
	#[deku(id = "0x42")]
	Byte,
	/// C
	#[deku(id = "0x43")]
	Char,
	/// D
	#[deku(id = "0x44")]
	Double,
	/// F
	#[deku(id = "0x46")]
	Float,
	/// I
	#[deku(id = "0x49")]
	Int,
	/// J
	#[deku(id = "0x4A")]
	Long,
	/// S
	#[deku(id = "0x53")]
	Short,
	/// Z
	#[deku(id = "0x5A")]
	Boolean,
}

impl From<char> for BaseType {
	fn from(value: char) -> Self {
		match value {
			'B' => BaseType::Byte,
			'C' => BaseType::Char,
			'D' => BaseType::Double,
			'F' => BaseType::Float,
			'I' => BaseType::Int,
			'J' => BaseType::Long,
			'S' => BaseType::Short,
			'Z' => BaseType::Boolean,
			_ => panic!("Invalid base type: {}", value),
		}
	}
}

/// Represents a parsed method descriptor that describes method parameters and return type.
///
/// Method descriptors follow the format: `(ParamTypes...)ReturnType`
/// For example:
/// - `()V` - Takes no parameters and returns void
/// - `(II)I` - Takes two ints and returns an int
/// - `([Ljava/lang/String;)V` - Takes String array, returns void (public static void main)
#[derive(Debug, PartialEq, Clone)]
pub struct MethodDescriptor {
	parameters: Vec<FieldType>,
	// none = void/v
	return_type: Option<FieldType>,
}

impl MethodDescriptor {
	fn void() -> Self {
		Self {
			parameters: vec![],
			return_type: None,
		}
	}
	fn psvm() -> Self {
		MethodDescriptor::parse("([Ljava/lang/String;)V").unwrap()
	}
}

impl Display for BaseType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			BaseType::Byte => write!(f, "B"),
			BaseType::Char => write!(f, "C"),
			BaseType::Double => write!(f, "D"),
			BaseType::Float => write!(f, "F"),
			BaseType::Int => write!(f, "I"),
			BaseType::Long => write!(f, "J"),
			BaseType::Short => write!(f, "S"),
			BaseType::Boolean => write!(f, "Z"),
		}
	}
}

impl Display for FieldType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			FieldType::Base(base) => write!(f, "{}", base),
			FieldType::ClassType(name) => write!(f, "L{};", name),
			FieldType::ArrayType(component) => write!(f, "[{}", component),
		}
	}
}

impl Display for MethodDescriptor {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "(")?;
		for param in &self.parameters {
			write!(f, "{}", param)?;
		}
		write!(f, ")")?;
		match &self.return_type {
			Some(ret) => write!(f, "{}", ret),
			None => write!(f, "V"),
		}
	}
}

/// Represents types that can be used for fields in the JVM.
///
/// Field types can be:
/// - Primitive types (represented by BaseType)
/// - Class types (prefixed with L)
/// - Array types (prefixed with [)
#[derive(Debug, PartialEq, Clone)]
pub enum FieldType {
	/// Represents a primitive type in the JVM, such as int, boolean, etc.
	/// These are stored directly on the stack rather than as object references.
	Base(BaseType),
	/// Represents a reference to a class type, prefixed with 'L' in descriptors.
	/// For example: "Ljava/lang/String;" represents a String reference.
	ClassType(String),
	/// Represents an array type, prefixed with '[' in descriptors.
	/// The inner FieldType represents the component type of the array.
	ArrayType(Box<FieldType>),
}

enum ExecutionResult {
	Continue,
	Return(()),
	ReturnValue(Value),
}
#[derive(Debug)]
enum VmError {
	ConstantPoolError(String),
	StackError(String),
	DekuError(DekuError),
	LoaderError(String),
	ExecutionError,
}

impl Display for VmError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			VmError::ConstantPoolError(msg) => write!(f, "Constant pool error: {}", msg),
			VmError::StackError(msg) => write!(f, "Stack error: {}", msg),
			VmError::DekuError(err) => write!(f, "Deku error: {}", err),
			VmError::LoaderError(msg) => write!(f, "Loader error: {}", msg),
			VmError::ExecutionError => write!(f, "Execution error"),
		}
	}
}

impl From<ConstantPoolError> for VmError {
	fn from(value: ConstantPoolError) -> Self {
		Self::ConstantPoolError(value.to_string())
	}
}
impl From<DekuError> for VmError {
	fn from(value: DekuError) -> Self {
		Self::DekuError(value)
	}
}

impl Frame {
	fn execute_instruction(&mut self, op: &Ops) -> Result<ExecutionResult, VmError> {
		match op {
			Ops::ldc(index) => {
				let thing = self.pool.get_constant(index.to_owned() as u16)?;
				println!("\tLoading constant: {}", thing);
				let resolved: Option<Value> = match thing {
					ConstantPoolEntry::Utf8(x) => {
						println!("{:?}", String::from_utf8(x.bytes.clone()));
						warn!("Utf8 loading not yet implemented");
						None
					}
					ConstantPoolEntry::Integer(x) => Some(Value::Int(x.clone())),
					ConstantPoolEntry::Float(x) => Some(Value::Float(x.clone())),
					ConstantPoolEntry::Class(x) => None,
					ConstantPoolEntry::String(x) => {
						warn!("String loading not yet implemented");
						None
					}

					ConstantPoolEntry::MethodHandle(x) => {
						warn!("Method handle loading not yet implemented");
						None
					}
					ConstantPoolEntry::MethodType(x) => {
						warn!("Method type loading not yet implemented");
						None
					}
					ConstantPoolEntry::Dynamic(x) => {
						warn!("Dynamic loading not yet implemented");
						None
					}
					_ => {
						panic!(
							"Cannot load constant, is not of loadable type: {:?}. ",
							thing
						);
						None
					}
				};
				if let Some(x) = resolved {
					self.stack.push(x);
				};
				Ok(ExecutionResult::Continue)
			}
			Ops::ldc2_w(index) => {
				let val = self.pool.get_constant(*index)?;
				println!("\tLoading constant: {}", val);
				let resolved = match val {
					ConstantPoolEntry::Double(x) => Some(Value::Double(x.clone())),
					ConstantPoolEntry::Long(x) => Some(Value::Long(x.clone())),
					_ => None,
				};
				if let Some(x) = resolved {
					self.stack.push(x);
				};
				Ok(ExecutionResult::Continue)
			}
			// store
			Ops::fstore(index) => {
				store!(self, f, *index as usize)
			}
			Ops::fstore_0 => {
				store!(self, f, 0)
			}
			Ops::fstore_1 => {
				store!(self, f, 1)
			}
			Ops::fstore_2 => {
				store!(self, f, 2)
			}
			Ops::fstore_3 => {
				store!(self, f, 3)
			}
			Ops::dstore(index) => {
				store!(self, d, *index as usize)
			}
			Ops::dstore_0 => {
				store!(self, d, 0)
			}
			Ops::dstore_1 => {
				store!(self, d, 1)
			}
			Ops::dstore_2 => {
				store!(self, d, 2)
			}
			Ops::dstore_3 => {
				store!(self, d, 3)
			}

			// load
			Ops::fload(index) => {
				load!(self, f, *index as usize)
			}
			Ops::fload_0 => {
				load!(self, f, 0)
			}
			Ops::fload_1 => {
				load!(self, f, 1)
			}
			Ops::fload_2 => {
				load!(self, f, 2)
			}
			Ops::fload_3 => {
				load!(self, f, 3)
			}
			Ops::dload(index) => {
				load!(self, d, *index as usize)
			}
			Ops::dload_0 => {
				load!(self, d, 0)
			}
			Ops::dload_1 => {
				load!(self, d, 1)
			}
			Ops::dload_2 => {
				load!(self, d, 2)
			}
			Ops::dload_3 => {
				load!(self, d, 3)
			}

			Ops::f2d => {
				if let Value::Float(float) = self.stack.pop().expect("Stack must have value") {
					let double: f64 = float.into();
					self.stack.push(Value::Double(double));
					Ok(ExecutionResult::Continue)
				} else {
					Err(VmError::StackError(
						"Popped value was not float".to_string(),
					))
				}
			}

			Ops::dadd => {
				let value1 = self.stack.pop().expect("Stack must have value");
				let value2 = self.stack.pop().expect("Stack must have value");
				if let (Value::Double(double1), Value::Double(double2)) =
					(value1.clone(), value2.clone())
				{
					self.stack.push(Value::Double(double1 + double2));
					Ok(ExecutionResult::Continue)
				} else {
					Err(VmError::StackError(format!(
						"{value1:?} or {value2:?} was not a double"
					)))
				}
			}

			// get static field
			// can init the field
			Ops::getstatic(index) => {
				let field_ref = self.pool.resolve_field(*index)?;
				println!("Getting static field {field_ref:?}");

				let init_class = self
					.thread
					.get_or_resolve_class(&field_ref.class, self.thread.clone())
					.expect("TO hecken work");
				let result = init_class
					.find_field(&field_ref.name, field_ref.desc)
					.expect("TO hecken work");
				let constant = result
					.value
					.lock()
					.unwrap()
					.clone()
					.expect("Static field was not initialised");
				self.stack.push(constant.into());

				// let (code, pool) = {
				// 	let mut loader = self.vm.loader.lock().unwrap();
				// 	let class = loader.get_or_load(&field.class).unwrap();
				// 	let field = class.get_static_field_value(&field)
				// 		// let code = class.get_code(meth)?;
				// 		(code, pool)
				// };
				// println!("{:?}", field);
				Ok(ExecutionResult::Continue)
			}

			Ops::invokevirtual(index) => {
				let meth = self.pool.resolve_method_ref(*index)?;
				let params = meth.desc.num_arguments();
				let last = self.stack.len() - 1;
				let first = last - params + 1;
				let slice = self.stack.get(first..last).unwrap().to_vec();
				//sub slice param length + one, throw it to frame new
				let (code, pool) = {
					let mut loader = self.thread.loader.lock().unwrap();
					let class = loader.get_or_load(&meth.class).unwrap();
					let pool = class.constant_pool.clone();
					let code = class
						.find_method(&meth.name, &meth.desc)
						.unwrap()
						.code
						.clone()
						.unwrap();
					(code, pool)
				};
				// let code = class.get_code(meth)?;
				// let class = self.vm.loader.get_or_load(&meth.class).unwrap();
				// let pool = &class.constant_pool;
				let vars = slice;
				let frame = Frame::new(code, pool.clone(), vars, self.thread.clone());
				// println!("{:?}", meth);
				// todo!("Finish invoke virtual");
				Ok(ExecutionResult::Continue)
			}

			Ops::invokestatic(index) => {
				let method_ref = self.pool.resolve_method_ref(*index)?;
				let class = self
					.thread
					.get_or_resolve_class(&method_ref.class, self.thread.clone())?;
				// let method_data = class
				// 	.find_method(&method_ref.name, method_ref.desc)?
				// 	.clone();

				let result = self.thread.invoke(method_ref, self.thread.clone())?;
				if let Some(val) = result {
					self.stack.push(val)
				}
				// todo!("Implement invoke static {}", index)
				Ok(ExecutionResult::Continue)
			}

			Ops::aconst_null => {
				self.stack.push(NULL);
				Ok(ExecutionResult::Continue)
			}

			Ops::putstatic(index) => {
				let field_ref = self.pool.resolve_field(*index)?;
				println!("Getting static field {field_ref:?}");

				let init_class = self
					.thread
					.get_or_resolve_class(&field_ref.class, self.thread.clone())
					.expect("TO hecken work");
				let result = init_class
					.find_field(&field_ref.name, field_ref.desc)
					.expect("TO hecken work");
				let value = self.stack.pop().expect("stack to have value");
				*result.value.lock().unwrap() = Some(value);
				Ok(ExecutionResult::Continue)
			}

			Ops::return_void => Ok(ExecutionResult::Return(())),
			_ => {
				todo!("Unimplemented op: {:?}", op)
			}
		}
	}
}
