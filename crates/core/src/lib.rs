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

use crate::value::{OperandStack, Primitive};
use crate::value::LocalVariables;
use crate::attributes::{ArrayType, Attribute, CodeAttribute};
use crate::class_file::constant_pool::ConstantPoolExt;
use crate::class_file::constant_pool::{ConstantPoolError, ConstantPoolGet};
use crate::class_file::{Bytecode, ClassFile, ConstantPoolEntry, MethodData, MethodRef};
use crate::objects::array::ArrayReference;
pub use crate::thread::VmThread;
use ::jni::sys::{jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
use deku::{DekuContainerRead, DekuError};
use deku_derive::{DekuRead, DekuWrite};
use env_logger::Builder;
use instructions::Ops;
use itertools::Itertools;
use log::{error, info, trace, warn, LevelFilter};
use objects::object::ReferenceKind;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::ops::{BitAnd, Deref};
use std::sync::{Arc, Mutex};
use value::{Value};
use vm::Vm;

mod attributes;
mod bimage;
mod class;
pub mod class_file;
mod class_loader;
mod instructions;
mod jni;
mod macros;
mod native_libraries;
pub mod objects;
mod prim;
mod rng;
mod thread;
pub mod value;
pub mod vm;
// const NULL: Value = Value::Reference(None);

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
/// pseudo main
pub fn run() {
	Builder::from_default_env()
		.filter_level(LevelFilter::Trace)
		.filter_module("deku", LevelFilter::Warn)
		.filter_module("roast_vm_core::class_file::class_file", LevelFilter::Info)
		.filter_module("roast_vm_core::attributes", LevelFilter::Info)
		.filter_module("roast_vm_core::instructions", LevelFilter::Info)
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
	// let mut vm = Vm::new("org/example/Main");

	// println!("{:?}", ops);
	// println!("{:?}", var_table.local_variable_table);
	// vm.method(ops.clone(), code, var_table);
}

// impl Value {
// 	fn Int(i: i32) -> Value {
// 		Value::Primitive(Primitive::Int(i))
// 	}
//
// 	fn Float(f: f32) -> Value {
// 		Value::Primitive(Primitive::Float(f))
// 	}
//
// 	fn Double(d: f64) -> Value {
// 		Value::Primitive(Primitive::Double(d))
// 	}
//
// 	fn Long(l: i64) -> Value {
// 		Value::Primitive(Primitive::Long(l))
// 	}
//
// 	fn Char(c: u16) -> Value {
// 		Value::Primitive(Primitive::Char(c))
// 	}
//
// 	fn Boolean(b: bool) -> Value {
// 		Value::Primitive(Primitive::Boolean(b))
// 	}
//
// 	fn Byte(b: i8) -> Value {
// 		Value::Primitive(Primitive::Byte(b))
// 	}
//
// 	fn Short(s: i16) -> Value {
// 		Value::Primitive(Primitive::Short(s))
// 	}
// }

/// Represents a JVM stack frame for method execution.
///
/// A frame contains all the execution state needed to run a single method:
/// - Program counter (PC) tracking the current bytecode instruction
/// - Operand stack for intermediate values during computation
/// - Local variables for method parameters and local vars
/// - Reference to the constant pool for the class
/// - The bytecode to execute
/// - Reference to the thread executing this frame
#[derive(Clone)]
struct Frame {
	/// Program counter - index of the current bytecode instruction
	pc: i64,
	/// Operand stack for intermediate values
	stack: OperandStack,
	/// Local variables (includes method parameters)
	vars: LocalVariables,
	/// Constant pool from the class file
	pool: Arc<Vec<ConstantPoolEntry>>,

	/// The bytecode instructions for this method
	bytecode: Bytecode,

	/// The thread executing this frame
	thread: Arc<VmThread>,
	// The mod being invoked
	method_ref: MethodRef,
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
		fn push(&mut self, val: Value) {
		match val.clone() {
			Value::Primitive(x) => match x {
				Primitive::Boolean(x) => {
					let i = x as i32;
					self.stack.push(i.into())
				}
				Primitive::Char(x) => {
					let i = x as i32;
					self.stack.push(i.into())
				}
				Primitive::Byte(x) => {
					let i = x as i32;
					self.stack.push(i.into())
				}
				Primitive::Short(x) => {
					let i = x as i32;
					self.stack.push(i.into())
				}
				_ => self.stack.push(val),
			},
			Value::Reference(_) => self.stack.push(val),
			Value::Padding => {panic!("We we pushing a pad?")}
		}
	}

	// fn load_constant(index: u8) {}
	fn new(
		method_ref: MethodRef,
		code_attr: CodeAttribute,
		pool: Arc<Vec<ConstantPoolEntry>>,
		mut locals: Vec<Value>,
		vm: Arc<Vm>,
	) -> Self {
		// Get current thread from thread-local storage
		let thread = VmThread::current(&vm);

		let max_stack = code_attr.max_stack as usize;
		let max_local = code_attr.max_locals as usize;
		let bytes = code_attr.code_length.to_be_bytes();
		let mut buf = Vec::new();
		buf.extend_from_slice(&bytes);
		buf.extend_from_slice(&code_attr.code.clone());
		let (_rest, bytecode) = Bytecode::from_bytes((buf.as_ref(), 0)).unwrap();
		Frame {
			pc: 0,
			stack: OperandStack::with_capacity(max_stack),
			vars: LocalVariables::from_args(locals, max_local),
			pool,
			bytecode,
			thread,
			method_ref,
		}
	}
	fn execute(&mut self) -> Result<Option<Value>, VmError> {
		let binding = self.bytecode.code.clone();
		loop {
			let (offset, op) = self.next().expect("No ops :(");
			info!("pre set: {}", self.pc);
			self.pc = offset as i64;
			info!("post set: {}", self.pc);
			trace!("Executing Op: {:?}", op);
			let result = self.execute_instruction(op.clone());
			match result {
				Ok(ExecutionResult::Return(())) => return Ok(None),
				Ok(ExecutionResult::ReturnValue(val)) => return Ok(Some(val)),
				Ok(ExecutionResult::Advance(offset)) => {
					info!("pre offset: {}", self.pc);
					self.pc += offset as i64;
					info!("post offset: {}", self.pc);
				}
				Ok(_) => self.pc += 1,
				Err(x) => {
					let objs = self.thread.gc.read().unwrap()
						.objects
						.iter()
						.map(|(x, y)| format!("{x} : {y}"))
						.collect::<Vec<_>>();
					let len = objs.len().clone();
					error!("Error in method: {:#?}", self.method_ref);
					error!("Heap dump: len: {len} objs:\n{objs:#?}");
					panic!("Mission failed, we'll get em next time:\n{x}")
				}
			}
			trace!(
				"State:\n\tStack: [{}]\n\tLocals: [{}]\n",
				self.stack
					.iter()
					.map(|v| v.to_string())
					.collect::<Vec<_>>()
					.join(", "),
				self.vars
					.iter()
					.map(|v| v.to_string())
					.collect::<Vec<_>>()
					.join(", ")
			);
		}
		Err(VmError::ExecutionError)
	}

	fn next(&mut self) -> Option<(u16, Ops)> {
		self.bytecode
			.code
			.iter()
			.find(|(offset, op)| *offset as i64 >= self.pc)
			.map(|(op)| op.clone())
	}
}

// impl IntoIterator for Frame {
// 	type Item = ();
// 	type IntoIter = ();
//
// 	fn into_iter(self) -> Self::IntoIter {
// 		self.bytecode.code.iter()
// 	}
// }

/// Unique identifier for a VM thread.
///
/// Each VmThread is assigned a unique ThreadId when created. This ID is used to:
/// - Track threads in the VM's thread registry (DashMap)
/// - Identify the current thread via thread-local storage
/// - Support future multi-threading when Java threads map to OS threads
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ThreadId(pub u64);

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

	pub fn arg_width(&self) -> usize {
		self.parameters.iter().fold(0, |acc, e| {
			acc + match e {
				FieldType::Base(base) => match base {
					BaseType::Byte => 1,
					BaseType::Char => 1,
					BaseType::Double => 2,
					BaseType::Float => 1,
					BaseType::Int => 1,
					BaseType::Long => 2,
					BaseType::Short => 1,
					BaseType::Boolean => 1,
				},
				FieldType::ClassType(_) => 1,
				FieldType::ArrayType(_) => 1,
			}
		})
	}

	pub fn param_string(&self) -> String {
		self.parameters.iter().join("")
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
	Advance(i16),
	Return(()),
	ReturnValue(Value),
}
#[derive(Debug)]
pub enum VmError {
	ConstantPoolError(String),
	StackError(String),
	InvariantError(String),
	DekuError(DekuError),
	LoaderError(String),
	ExecutionError,
	NativeError(String),
}

impl VmError {
	pub fn stack_not_int() -> Self {
		Self::InvariantError("Value on stack was not an int".to_string())
	}
}

impl Display for VmError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			VmError::ConstantPoolError(msg) => write!(f, "Constant pool error: {}", msg),
			VmError::StackError(msg) => write!(f, "Stack error: {}", msg),
			VmError::InvariantError(msg) => write!(f, "Invariant error: {}", msg),
			VmError::DekuError(err) => write!(f, "Deku error: {}", err),
			VmError::LoaderError(msg) => write!(f, "Loader error: {}", msg),
			VmError::ExecutionError => write!(f, "Execution error"),
			VmError::NativeError(msg) => write!(f, "Native error {msg}"),
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
	fn pop(&mut self) -> Result<Value, VmError>{
		self.stack.pop()
	}
	fn execute_instruction(&mut self, op: Ops) -> Result<ExecutionResult, VmError> {
		match op {
			Ops::nop => {
				// TODO Should nop have any side effects?
				warn!("TODO Should nop have any side effects? Investigate.");
				Ok(ExecutionResult::Continue)
			}
			// Constants
			Ops::aconst_null => {
				self.stack.push(Value::NULL);
				Ok(ExecutionResult::Continue)
			}

			Ops::iconst_m1 => {
				self.stack.push(Value::from(-1i32));
				Ok(ExecutionResult::Continue)
			}

			Ops::iconst_0 => {
				self.stack.push(Value::from(0i32));
				Ok(ExecutionResult::Continue)
			}

			Ops::iconst_1 => {
				self.stack.push(Value::from(1i32));
				Ok(ExecutionResult::Continue)
			}

			Ops::iconst_2 => {
				self.stack.push(Value::from(2i32));
				Ok(ExecutionResult::Continue)
			}

			Ops::iconst_3 => {
				self.stack.push(Value::from(3i32));
				Ok(ExecutionResult::Continue)
			}

			Ops::iconst_4 => {
				self.stack.push(4.into());
				Ok(ExecutionResult::Continue)
			}

			Ops::iconst_5 => {
				self.stack.push(5.into());
				Ok(ExecutionResult::Continue)
			}
			Ops::lconst_0 => {
				self.stack.push(0i64.into());
				Ok(ExecutionResult::Continue)
			}
			Ops::lconst_1 => {
				self.stack.push(1i64.into());
				Ok(ExecutionResult::Continue)
			}
			Ops::fconst_0 => {
				self.stack.push(0f32.into());
				Ok(ExecutionResult::Continue)
			}
			Ops::fconst_1 => {
				self.stack.push(1f32.into());
				Ok(ExecutionResult::Continue)
			}
			Ops::fconst_2 => {
				self.stack.push(2f32.into());
				Ok(ExecutionResult::Continue)
			}
			Ops::dconst_0 => {
				self.stack.push(0f64.into());
				Ok(ExecutionResult::Continue)
			}
			Ops::dconst_1 => {
				self.stack.push(1f64.into());
				Ok(ExecutionResult::Continue)
			}
			Ops::bipush(byte) => {
				self.stack.push((byte as i32).into());
				Ok(ExecutionResult::Continue)
			}
			Ops::sipush(short) => {
				self.stack.push((short as i32).into());
				Ok(ExecutionResult::Continue)
			}
			Ops::ldc(index) => self.load_constant(index as u16),
			Ops::ldc_w(index) => self.load_constant(index),

			Ops::ldc2_w(index) => {
				let val = self.pool.get_constant(index)?;
				trace!("\tLoading constant: {}", val);
				let resolved = match val {
					ConstantPoolEntry::Double(x) => Some(Value::from(*x)),
					ConstantPoolEntry::Long(x) => Some(Value::from(*x)),
					_ => None,
				};
				if let Some(x) = resolved {
					self.stack.push(x);
					// on second thoughts, i dont think that's right
					// self.stack.push(Value::Reference(None));
				};
				Ok(ExecutionResult::Continue)
			}

			// loads

			//iload
			Ops::iload(index) => {
				load!(self, i, index as usize)
			}
			Ops::iload_0 => {
				load!(self, i, 0)
			}
			Ops::iload_1 => {
				load!(self, i, 1)
			}
			Ops::iload_2 => {
				load!(self, i, 2)
			}
			Ops::iload_3 => {
				load!(self, i, 3)
			}
			Ops::lload(index) => {
				load!(self, l, index as usize)
			}
			Ops::lload_0 => {
				load!(self, l, 0)
			}
			Ops::lload_1 => {
				load!(self, l, 1)
			}
			Ops::lload_2 => {
				load!(self, l, 2)
			}
			Ops::lload_3 => {
				load!(self, l, 3)
			}
			Ops::fload(index) => {
				load!(self, f, index as usize)
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
				load!(self, d, index as usize)
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
			Ops::aload(index) => {
				load!(self, a, index as usize)
			}
			Ops::aload_0 => {
				load!(self, a, 0)
			}
			Ops::aload_1 => {
				load!(self, a, 1)
			}
			Ops::aload_2 => {
				load!(self, a, 2)
			}
			Ops::aload_3 => {
				load!(self, a, 3)
			}
			Ops::iaload => {
				let Value::Primitive(Primitive::Int(index)) =
					self.stack.pop().expect("value on stack")
				else {
					panic!("index on stack was not int")
				};
				let Value::Reference(Some(ReferenceKind::ArrayReference(ArrayReference::Int(arr)))) =
					self.stack.pop().expect("value on stack")
				else {
					panic!("Reference not on stack or not an int array")
				};
				let i = arr.lock().unwrap().get(index);
				self.stack.push(Value::from(i));
				Ok(ExecutionResult::Continue)
			}
			Ops::laload => {
				todo!("long array load")
			}
			Ops::faload => {
				todo!("float array load")
			}
			Ops::daload => {
				todo!("double array load")
			}
			Ops::aaload => {
				let Value::Primitive(Primitive::Int(index)) =
					self.stack.pop().expect("value on stack")
				else {
					panic!("index on stack was not int")
				};
				let Value::Reference(Some(ReferenceKind::ArrayReference(ArrayReference::Object(
					arr,
				)))) = self.stack.pop().expect("value on stack")
				else {
					panic!("Reference not on stack or not a reference array")
				};
				let reference = arr.lock().unwrap().get(index);
				self.stack.push(Value::from(reference));
				Ok(ExecutionResult::Continue)
			}
			Ops::baload => {
				todo!("boolean array load")
			}
			Ops::caload => {
				let Value::Primitive(Primitive::Int(index)) = self.pop()? else {
					panic!("index on stack was not int")
				};
				let Value::Reference(Some(ReferenceKind::ArrayReference(ArrayReference::Char(
					arr,
				)))) = self.pop()?
				else {
					panic!("Reference not on stack or not a char array")
				};
				let c = arr.lock().unwrap().get(index);
				self.push(Value::from(c));
				Ok(ExecutionResult::Continue)
			}
			Ops::saload => {
				todo!("short array load")
			}

			// store
			Ops::istore(index) => {
				store!(self, i, index as usize)
			}
			Ops::istore_0 => {
				store!(self, i, 0)
			}
			Ops::istore_1 => {
				store!(self, i, 1)
			}
			Ops::istore_2 => {
				store!(self, i, 2)
			}
			Ops::istore_3 => {
				store!(self, i, 3)
			}

			Ops::fstore(index) => {
				store!(self, f, index as usize)
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
				store!(self, d, index as usize)
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

			Ops::lstore(index) => {
				store!(self, l, index as usize)
			}
			Ops::lstore_0 => {
				store!(self, l, 0)
			}
			Ops::lstore_1 => {
				store!(self, l, 1)
			}
			Ops::lstore_2 => {
				store!(self, l, 2)
			}
			Ops::lstore_3 => {
				store!(self, l, 3)
			}

			Ops::astore(index) => {
				store!(self, a, index as usize)
			}
			Ops::astore_0 => {
				store!(self, a, 0)
			}
			Ops::astore_1 => {
				store!(self, a, 1)
			}
			Ops::astore_2 => {
				store!(self, a, 2)
			}
			Ops::astore_3 => {
				store!(self, a, 3)
			}
			Ops::iastore => array_store!(self, Int, Int),
			Ops::lastore => array_store!(self, Long, Long),
			Ops::fastore => array_store!(self, Float, Float),
			Ops::dastore => array_store!(self, Double, Double),
			Ops::aastore => {
				let Value::Reference((value)) = self.pop()? else {
					panic!("Value on stack was not ref")
				};
				let Value::Primitive(Primitive::Int(index)) = self.pop()? else {
					panic!("index on stack was not int")
				};
				let Value::Reference(Some(ReferenceKind::ArrayReference(ArrayReference::Object(
					arr,
				)))) = self.pop()?
				else {
					panic!("Reference not on stack or not an int array")
				};
				arr.lock().unwrap().set(index, value);
				Ok(ExecutionResult::Continue)
			}
			Ops::bastore => array_store_cast!(self, Byte, i8),
			Ops::castore => array_store_cast!(self, Char, jchar),
			Ops::sastore => array_store_cast!(self, Short, i16),

			//Stack
			Ops::pop => {
				let v1 = self.pop()?;
				if v1.is_wide() {
					Err(VmError::InvariantError("Op:pop on single wide value".to_string()))
				} else { Ok(ExecutionResult::Continue) }
			}
			Ops::pop2 => {
				let peek = self.stack.peek()?;
				if peek.is_wide() {
					let _v1 = self.pop()?;
					Ok(ExecutionResult::Continue)
				} else {
					let _v2 = self.pop()?;
					let v1 = self.pop()?;
					if v1.is_wide() {
						Err(VmError::InvariantError("Op:pop2 popped a 2wide on second pop".to_string()))
					} else { Ok(ExecutionResult::Continue) }
				}
			}
			Ops::dup_x1 => {
				let v1 = self.pop()?;
				let v2 = self.pop()?;
				if v1.is_wide() || v2.is_wide() {
					Err(VmError::InvariantError("dup_x1 operated on 2 wide value".to_string()))
				} else {
					self.push(v1.clone());
					self.push(v2);
					self.push(v1);
					Ok(ExecutionResult::Continue)
				}
			}
			Ops::dup_x2 => {
				let v1 = self.pop()?;
				let v2 = self.pop()?;
				if v1.is_wide() {
					Err(VmError::InvariantError("dup_x2 operated on 1st 2 wide value".to_string()))
				} else if v2.is_wide() {
					self.push(v1.clone());
					self.push(v2);
					self.push(v1);
					Ok(ExecutionResult::Continue)
				} else if self.stack.peek()?.is_wide() {
					Err(VmError::InvariantError("dup_x2 operated on 3rd 2 wide value".to_string()))
				}
				else {
					let v3 = self.pop()?;
					self.push(v1.clone());
					self.push(v3);
					self.push(v2);
					self.push(v1);
					Ok(ExecutionResult::Continue)
				}
			}
			Ops::dup => {
				if let Ok(value) = self.stack.peek() {
					self.stack.push(value.clone());
					Ok(ExecutionResult::Continue)
				} else {
					Err(VmError::StackError("Stack underflow".to_string()))
				}
			}
			Ops::dup2 => {
				let v1 = self.pop()?;
				if v1.is_wide() {
					self.push(v1.clone());
					self.push(v1);
					Ok(ExecutionResult::Continue)
				} else if self.stack.peek()?.is_wide() {
					Err(VmError::InvariantError("dup2 operated on 2nd, 2 wide value".to_string()))
				} else {
					let v2 = self.pop()?;
					self.push(v2.clone());
					self.push(v1.clone());
					self.push(v2);
					self.push(v1);
					Ok(ExecutionResult::Continue)
				}
			}
			Ops::dup2_x1 => {
				let v1 = self.pop()?;
				if v1.is_wide() {
					// Form 2: v2, v1 → v1, v2, v1
					// v1 is category 2, v2 must be category 1
					let v2 = self.pop()?;
					if v2.is_wide() {
						Err(VmError::InvariantError("dup2_x1 form 2: v2 must be category 1".to_string()))
					} else {
						self.push(v1.clone());
						self.push(v2);
						self.push(v1);
						Ok(ExecutionResult::Continue)
					}
				} else {
					// Form 1: v3, v2, v1 → v2, v1, v3, v2, v1
					// all must be category 1
					let v2 = self.pop()?;
					if v2.is_wide() {
						Err(VmError::InvariantError("dup2_x1 form 1: v2 must be category 1".to_string()))
					} else {
						let v3 = self.pop()?;
						if v3.is_wide() {
							Err(VmError::InvariantError("dup2_x1 form 1: v3 must be category 1".to_string()))
						} else {
							self.push(v2.clone());
							self.push(v1.clone());
							self.push(v3);
							self.push(v2);
							self.push(v1);
							Ok(ExecutionResult::Continue)
						}
					}
				}
			}
			Ops::dup2_x2 => {
				let v1 = self.pop()?;
				let v2 = self.pop()?;

				if v1.is_wide() {
					if v2.is_wide() {
						// Form 4: v2, v1 → v1, v2, v1 (both category 2)
						self.push(v1.clone());
						self.push(v2);
						self.push(v1);
						Ok(ExecutionResult::Continue)
					} else {
						// Form 2: v3, v2, v1 → v1, v3, v2, v1
						// v1 is category 2, v2 and v3 are category 1
						let v3 = self.pop()?;
						if v3.is_wide() {
							Err(VmError::InvariantError("dup2_x2 form 2: v3 must be category 1".to_string()))
						} else {
							self.push(v1.clone());
							self.push(v3);
							self.push(v2);
							self.push(v1);
							Ok(ExecutionResult::Continue)
						}
					}
				} else if v2.is_wide() {
					// v1 category 1, v2 category 2 - no valid form for this
					Err(VmError::InvariantError("dup2_x2: invalid - v1 cat1, v2 cat2".to_string()))
				} else {
					// v1 and v2 are both category 1
					let v3 = self.pop()?;
					if v3.is_wide() {
						// Form 3: v3, v2, v1 → v2, v1, v3, v2, v1
						// v3 is category 2
						self.push(v2.clone());
						self.push(v1.clone());
						self.push(v3);
						self.push(v2);
						self.push(v1);
						Ok(ExecutionResult::Continue)
					} else {
						// Form 1: v4, v3, v2, v1 → v2, v1, v4, v3, v2, v1
						// all category 1
						let v4 = self.pop()?;
						if v4.is_wide() {
							Err(VmError::InvariantError("dup2_x2 form 1: v4 must be category 1".to_string()))
						} else {
							self.push(v2.clone());
							self.push(v1.clone());
							self.push(v4);
							self.push(v3);
							self.push(v2);
							self.push(v1);
							Ok(ExecutionResult::Continue)
						}
					}
				}
			}
			Ops::swap => {
				let v1 = self.pop()?;
				let v2 = self.pop()?;
				if v1.is_wide() || v2.is_wide() {
					Err(VmError::InvariantError("swap operated on 2 wide value".to_string()))
				} else {
					self.push(v1);
					self.push(v2);
					Ok(ExecutionResult::Continue)
				}
			}

			// Math
			// Addition
			Ops::iadd => binary_op!(self, Int, |a, b| a.wrapping_add(b)),
			Ops::ladd => binary_op!(self, Long, |a, b| a.wrapping_add(b)),
			Ops::fadd => binary_op!(self, Float, |a, b| a + b),
			Ops::dadd => binary_op!(self, Double, |a, b| a + b),

			// Subtraction
			Ops::isub => binary_op!(self, Int, |a, b| a.wrapping_sub(b)),
			Ops::lsub => binary_op!(self, Long, |a, b| a.wrapping_sub(b)),
			Ops::fsub => binary_op!(self, Float, |a, b| a - b),
			Ops::dsub => binary_op!(self, Double, |a, b| a - b),

			// Multiplication
			Ops::imul => binary_op!(self, Int, |a, b| a.wrapping_mul(b)),
			Ops::lmul => binary_op!(self, Long, |a, b| a.wrapping_mul(b)),
			Ops::fmul => binary_op!(self, Float, |a, b| a * b),
			Ops::dmul => binary_op!(self, Double, |a, b| a * b),

			// Division
			Ops::idiv => int_div_rem!(self, Int, i32, /),
			Ops::ldiv => int_div_rem!(self, Long, i64, /),
			Ops::fdiv => binary_op!(self, Float, |a, b| a / b),
			Ops::ddiv => binary_op!(self, Double, |a, b| a / b),

			// Remainder
			Ops::irem => int_div_rem!(self, Int, i32, %),
			Ops::lrem => int_div_rem!(self, Long, i64, %),
			Ops::frem => binary_op!(self, Float, |a, b| a % b),
			Ops::drem => binary_op!(self, Double, |a, b| a % b),

			// Negation
			Ops::ineg => unary_op!(self, Int, |v| v.wrapping_neg()),
			Ops::lneg => unary_op!(self, Long, |v| v.wrapping_neg()),
			Ops::fneg => unary_op!(self, Float, |v| -v),
			Ops::dneg => unary_op!(self, Double, |v| -v),

			// Shifts
			Ops::ishl => shift_op!(self, Int, 0x1f, |v, s| v << s),
			Ops::lshl => shift_op!(self, Long, 0x3f, |v, s| v << s),
			Ops::ishr => shift_op!(self, Int, 0x1f, |v, s| v >> s),
			Ops::lshr => shift_op!(self, Long, 0x3f, |v, s| v >> s),
			Ops::iushr => shift_op!(self, Int, 0x1f, |v, s| ((v as u32) >> s) as i32),
			Ops::lushr => shift_op!(self, Long, 0x3f, |v, s| ((v as u64) >> s) as i64),

			// Bitwise
			Ops::iand => binary_op!(self, Int, |a, b| a & b),
			Ops::land => binary_op!(self, Long, |a, b| a & b),
			Ops::ior => binary_op!(self, Int, |a, b| a | b),
			Ops::lor => binary_op!(self, Long, |a, b| a | b),
			Ops::ixor => binary_op!(self, Int, |a, b| a ^ b),
			Ops::lxor => binary_op!(self, Long, |a, b| a ^ b),

			Ops::iinc(index, increment) => {
				if let Value::Primitive(Primitive::Int(int)) = self.vars.get(index as usize) {
					let new_val = int + increment as i32;
					self.vars.set(index as usize, new_val.into());
					Ok(ExecutionResult::Continue)
				} else {
					Err(VmError::InvariantError("iinc requires integer value".to_string()))
				}
			}

			// Conversions
			Ops::i2l => convert_simple!(self, Int, i64),
			Ops::i2f => convert_simple!(self, Int, f32),
			Ops::i2d => convert_simple!(self, Int, f64),
			Ops::l2i => convert_simple!(self, Long, i32),
			Ops::l2f => convert_simple!(self, Long, f32),
			Ops::l2d => convert_simple!(self, Long, f64),

			Ops::f2i => convert_float_to_int!(self, Float, f32, i32),
			Ops::f2l => convert_float_to_int!(self, Float, f32, i64),
			Ops::f2d => convert_simple!(self, Float, f64),

			Ops::d2i => convert_float_to_int!(self, Double, f64, i32),
			Ops::d2l => convert_float_to_int!(self, Double, f64, i64),
			Ops::d2f => convert_simple!(self, Double, f32),

			Ops::i2b => convert_int_narrow!(self, i8),
			Ops::i2c => convert_int_narrow!(self, u16),
			Ops::i2s => convert_int_narrow!(self, i16),
			// Comparisons
			Ops::lcmp => {
				let Value::Primitive(Primitive::Long(v2)) = self.pop()? else {
					return Err(VmError::StackError("Expected long".into()));
				};
				let Value::Primitive(Primitive::Long(v1)) = self.pop()? else {
					return Err(VmError::StackError("Expected long".into()));
				};

				let result: i32 = match v1.cmp(&v2) {
					std::cmp::Ordering::Greater => 1,
					std::cmp::Ordering::Equal => 0,
					std::cmp::Ordering::Less => -1,
				};

				self.stack.push(Value::from(result));
				Ok(ExecutionResult::Continue)
			}
			Ops::fcmpl => float_cmp!(self, Float, -1),
			Ops::fcmpg => float_cmp!(self, Float, 1),
			Ops::dcmpl => float_cmp!(self, Double, -1),
			Ops::dcmpg => float_cmp!(self, Double, 1),

			Ops::ifeq(offset) => if_int_zero!(self, offset, ==),
			Ops::ifne(offset) => if_int_zero!(self, offset, !=),
			Ops::iflt(offset) => if_int_zero!(self, offset, <),
			Ops::ifge(offset) => if_int_zero!(self, offset, >=),
			Ops::ifgt(offset) => if_int_zero!(self, offset, >),
			Ops::ifle(offset) => if_int_zero!(self, offset, <=),
			Ops::if_icmpeq(offset) => if_int_cmp!(self, offset, ==),
			Ops::if_icmpne(offset) => if_int_cmp!(self, offset, !=),
			Ops::if_icmplt(offset) => if_int_cmp!(self, offset, <),
			Ops::if_icmpge(offset) => if_int_cmp!(self, offset, >=),
			Ops::if_icmpgt(offset) => if_int_cmp!(self, offset, >),
			Ops::if_icmple(offset) => if_int_cmp!(self, offset, <=),

			Ops::if_acmpeq(offset) => {
				if let Value::Reference(Some(value2)) = self.pop()? &&
					let Value::Reference(Some(value1)) = self.pop()?
				{
					if value1.id() == value2.id() {
						Ok(ExecutionResult::Advance(offset))
					} else {
						Ok(ExecutionResult::Continue)
					}
				} else {
					Err(VmError::stack_not_int())
				}
			}
			Ops::if_acmpne(offset) => {
				if let Value::Reference(Some(value2)) = self.pop()? &&
					let Value::Reference(Some(value1)) = self.pop()?
				{
					if value1.id() != value2.id() {
						Ok(ExecutionResult::Advance(offset))
					} else {
						Ok(ExecutionResult::Continue)
					}
				} else {
					Err(VmError::stack_not_int())
				}
			}
			// Control
			Ops::goto(offset) => {
				Ok(ExecutionResult::Advance(offset))
			},
			Ops::jsr(_) => {
				todo!("jsr")
			}
			Ops::ret(_) => {
				todo!("ret")
			}
			Ops::tableswitch => {
				todo!("tableswitch")
			}
			Ops::lookupswitch => {
				todo!("lookupswitch")
			}
			Ops::ireturn => {
				let x: i32 = match self.pop()? {
					Value::Primitive(Primitive::Int(v)) => v,
					Value::Primitive(Primitive::Boolean(v)) => {
						if v {
							1
						} else {
							0
						}
					}
					Value::Primitive(Primitive::Byte(v)) => v as i32,
					Value::Primitive(Primitive::Char(v)) => v as i32,
					Value::Primitive(Primitive::Short(v)) => v as i32,
					_ => {
						return Err(VmError::InvariantError(
							"ireturn requires integer-compatible value".to_owned(),
						))
					}
				};

				match &self.method_ref.desc.return_type {
					Some(FieldType::Base(base_type)) => match base_type {
						BaseType::Boolean => Ok(ExecutionResult::ReturnValue((x != 0).into())),
						BaseType::Byte => Ok(ExecutionResult::ReturnValue((x as i8).into())),
						BaseType::Char => Ok(ExecutionResult::ReturnValue((x as u16).into())),
						BaseType::Short => Ok(ExecutionResult::ReturnValue((x as i16).into())),
						BaseType::Int => Ok(ExecutionResult::ReturnValue(x.into())),
						_ => Err(VmError::InvariantError(
							"wrong return instruction for method".to_owned(),
						)),
					},
					_ => Err(VmError::InvariantError(
						"wrong return instruction for method".to_owned(),
					)),
				}
			}
			Ops::lreturn => {
				let val = self.pop()?;
				match val {
					Value::Primitive(Primitive::Long(_)) => Ok(ExecutionResult::ReturnValue(val)),
					_ => Err(VmError::StackError("Expected reference".into())),
				}
			}
			Ops::freturn => {
				let val = self.pop()?;
				match val {
					Value::Primitive(Primitive::Float(_)) => Ok(ExecutionResult::ReturnValue(val)),
					_ => Err(VmError::StackError("Expected reference".into())),
				}
			}
			Ops::dreturn => {
				let val = self.pop()?;
				match val {
					Value::Primitive(Primitive::Double(_)) => Ok(ExecutionResult::ReturnValue(val)),
					_ => Err(VmError::StackError("Expected reference".into())),
				}
			}
			Ops::areturn => {
				let val = self.pop()?;
				match val {
					Value::Reference(_) => Ok(ExecutionResult::ReturnValue(val)),
					_ => Err(VmError::StackError("Expected reference".into())),
				}
			}
			Ops::return_void => Ok(ExecutionResult::Return(())),

			// References

			// get static field
			// can init the field
			Ops::getstatic(index) => {
				let field_ref = self.pool.resolve_field(index)?;
				println!("Getting static field {field_ref:?}");

				let init_class = self
					.thread
					.get_or_resolve_class(&field_ref.class)
					.expect("TO hecken work");
				let result = init_class
					.find_field(&field_ref.name, &field_ref.desc)
					.expect("TO hecken work");
				let constant = result
					.value
					.lock()
					.unwrap()
					.clone()
					.expect("Static field was not initialised");
				self.push(constant);
				Ok(ExecutionResult::Continue)
			}

			Ops::putstatic(index) => {
				let field_ref = self.pool.resolve_field(index)?;
				trace!("Putting static field {field_ref:?}");

				let init_class = self
					.thread
					.get_or_resolve_class(&field_ref.class)
					.expect("TO hecken work");
				let static_field = init_class
					.find_field(&field_ref.name, &field_ref.desc)
					.expect("TO hecken work");
				let value = self.stack.pop().expect("stack to have value");
				*static_field.value.lock().unwrap() = Some(value);
				Ok(ExecutionResult::Continue)
			}

			Ops::getfield(index) => {
				let field_ref = self.pool.resolve_field(index)?;
				trace!("Getting field {field_ref:?}");
				let popped = self.pop()?;
				match popped {
					Value::Primitive(x) => {
						Err(VmError::StackError("Getfield era".parse().unwrap()))
					}
					Value::Reference(x) => {
						match x {
							None => Ok(ExecutionResult::Continue),
							Some(kind) => match kind {
								ReferenceKind::ObjectReference(x) => {
									let val = x.lock().unwrap().get_field(&field_ref);
									self.push(val);

									Ok(ExecutionResult::Continue)
								}
								ReferenceKind::ArrayReference(_) => {
									Err(VmError::StackError("get field".parse().unwrap()))
								}
							},
						}
						// self.stack.push(val);
						// Ok(ExecutionResult::Continue)
					}
					Value::Padding => {panic!("Uhh not possible chief")}
				}
			}

			Ops::putfield(index) => {
				let field_ref = self.pool.resolve_field(index)?;
				trace!("Setting field {field_ref:?}");
				let value = self.pop()?;
				{
					let value = value.clone();
					let ref_type = field_ref.desc;

					// match field_ref.desc {
					// 	FieldType::Base(x) => {
					//
					// 	}
					// 	FieldType::ClassType(x) => {
					//
					// 	}
					// 	FieldType::ArrayType(x) => {
					// 		x
					// 	}
					// }
					// debug_assert_eq!(
					// 	value, ref_type,
					// 	"popped:{} not equal to desired:{}",
					// 	value, ref_type
					// )
				}
				if let Value::Reference(reference) = self.stack.pop().expect("object on stack") {
					if let Some(ReferenceKind::ObjectReference(object)) = reference {
						object.lock().unwrap().set_field(&field_ref.name, value);
						Ok(ExecutionResult::Continue)
					} else {
						Err(VmError::StackError("Null pointer exception".to_string()))
					}
				} else {
					Err(VmError::StackError(
						"putfield tried to operate on a non object stack value".to_string(),
					))
				}

				// todo!("op putfield: index - {}", index)
			}
			Ops::invokevirtual(index) => {
				let method_ref = self.pool.resolve_method_ref(index)?;
				// the 1 represents the receiver
				let args_count = method_ref.desc.arg_width() + 1;
				let args = self.stack.pop_n(args_count)?;
				let result = self.thread.invoke(method_ref, args)?;
				if let Some(val) = result {
					self.push(val)
				}
				// todo!("Finish invoke virtual");
				Ok(ExecutionResult::Continue)
			}

			Ops::invokespecial(index) => {
				let method_ref = self.pool.resolve_method_ref(index)?;
				let class = self.thread.get_or_resolve_class(&method_ref.class)?;

				// the 1 represents the receiver
				let args_count = method_ref.desc.arg_width() + 1;
				let args = self.stack.pop_n(args_count)?;

				let result = self.thread.invoke(method_ref, args)?;
				if let Some(val) = result {
					self.push(val)
				}
				// todo!("invoke special");
				Ok(ExecutionResult::Continue)
			}

			Ops::invokestatic(index) => {
				let method_ref = self.pool.resolve_method_ref(index)?;
				let class = self.thread.get_or_resolve_class(&method_ref.class)?;

				let args_count = method_ref.desc.parameters.len();
				let args = self.stack.pop_n(args_count)?;

				let result = self.thread.invoke(method_ref, args)?;
				if let Some(val) = result {
					self.push(val)
				}
				Ok(ExecutionResult::Continue)
			}

			Ops::invokeinterface(_, _, _) => {
				todo!("invokeInterface")
			}

			Ops::invokedynamic(_, _) => {
				todo!("invokeDynamic")
			}

			// can init class
			Ops::new(index) => {
				let class = self.pool.resolve_class_name(index)?;

				let init_class = self
					.thread
					.get_or_resolve_class(&class)
					.expect("TO hecken work");
				let object = self.thread.gc.write().unwrap().new_object(init_class);
				self.stack
					.push(Value::Reference(Some(ReferenceKind::from(object))));
				Ok(ExecutionResult::Continue)
			}

			Ops::newarray(array_type) => {
				let value = self.stack.pop().expect("value to have stack");
				let Value::Primitive(Primitive::Int(count)) = value else {
					panic!("stack item was not int")
				};
				let array = self
					.thread
					.gc
					.write()
					.unwrap()
					.new_primitive_array(array_type.clone(), count);
				self.stack
					.push(Value::Reference(Some(ReferenceKind::ArrayReference(array))));
				Ok(ExecutionResult::Continue)
			}
			Ops::anewarray(index) => {
				let class_name = self.pool.resolve_class_name(index)?;
				println!("{}", class_name);
				// let array_class = self.thread.loader.get_or_create_array_class(class_name)?;
				let value = self.stack.pop().expect("value to have stack");
				let Value::Primitive(Primitive::Int(count)) = value else {
					panic!("stack item was not int")
				};
				let array = self.thread.gc.write().unwrap().new_object_array(count);
				self.stack
					.push(Value::Reference(Some(ReferenceKind::ArrayReference(array))));
				Ok(ExecutionResult::Continue)
			}
			Ops::arraylength => {
				let Value::Reference(Some(ReferenceKind::ArrayReference((array)))) =
					self.stack.pop().expect("value on stack")
				else {
					panic!("Reference not on stack or not an array")
				};
				self.push(Value::from(array.len()));
				Ok(ExecutionResult::Continue)
			}
			Ops::athrow => {
				todo!("athrow")
			}
			Ops::checkcast(index) => {
				let thing = self.pool.resolve_class_name(index)?;
				let into_class = self.thread.get_class(&thing)?;
				let popped = self.pop()?;
				if let Value::Reference(Some(x)) = popped.clone() {
					match x {
						ReferenceKind::ObjectReference(obj) => {
							if obj.lock().unwrap().class.is_assignable_into(into_class) { self.push(popped); Ok(ExecutionResult::Continue) } else { todo!("Error path") }

						}
						ReferenceKind::ArrayReference(arr) => {
							todo!("Arrays")
						}
					}
				} else { self.push(popped); Ok(ExecutionResult::Continue) }
			}
			Ops::instanceof(index) => {
				let thing = self.pool.resolve_class_name(index)?;
				let into_class = self.thread.get_class(&thing)?;
				let popped = self.pop()?;
				if let Value::Reference(Some(x)) = popped {
					match x {
						ReferenceKind::ObjectReference(obj) => {
							if obj.lock().unwrap().class.is_assignable_into(into_class) { self.push(1i32.into()) } else { self.push(0i32.into()) }
							Ok(ExecutionResult::Continue)
						}
						ReferenceKind::ArrayReference(arr) => {
							todo!("Arrays")
						}
					}
				} else { panic!("yeet") }
			}
			Ops::monitorenter => {
				todo!("monitorenter")
			}
			Ops::monitorexit => {
				todo!("monitorexit")
			}

			Ops::wide => {
				todo!("wide")
			}
			Ops::multianewarray(_, _) => {
				todo!("multianewarray")
			}
			Ops::ifnull(offset) => {
				if let Value::Reference(value) = self.pop()? {
					if value.is_none() {
						Ok(ExecutionResult::Advance(offset))
					} else {
						Ok(ExecutionResult::Continue)
					}
				} else {
					Err(VmError::stack_not_int())
				}
			}
			Ops::ifnonnull(offset) => {
				if let Value::Reference(value) = self.pop()? {
					if value.is_some() {
						Ok(ExecutionResult::Advance(offset))
					} else {
						Ok(ExecutionResult::Continue)
					}
				} else {
					Err(VmError::stack_not_int())
				}
			}
			Ops::goto_w(_) => {
				todo!("goto_w")
			}
			Ops::jsr_w(_) => {
				todo!("jsr_w")
			}
			Ops::breakpoint => {
				todo!("breakpoint")
			}
			Ops::impdep1 => {
				todo!("impdep1")
			}
			Ops::impdep2 => {
				todo!("impdep2")
			}
		}
	}

	fn load_constant(&mut self, index: u16) -> Result<ExecutionResult, VmError> {
		let thing = self.pool.get_constant(index.to_owned())?;
		trace!("\tLoading constant: {}", thing);
		let resolved = match thing {
			ConstantPoolEntry::Integer(x) => Value::from(*x),
			ConstantPoolEntry::Float(x) => Value::from(*x),
			ConstantPoolEntry::Class(x) => {
				let name = self.pool.get_string(x.name_index)?;
				let class = self.thread.get_or_resolve_class(&name)?;
				let class_ref = self.thread.gc.read().unwrap().get(*class.mirror.wait());
				Value::from(class_ref)
			}
			ConstantPoolEntry::String(x) => {
				let utf_ref = self.pool.get_string(x.string_index)?;
				trace!("{utf_ref}");
				let string_class = self.thread.get_or_resolve_class("java/lang/String")?;
				let string_ref = self.thread.intern_string(&utf_ref);
				Value::from(string_ref)
			}

			ConstantPoolEntry::MethodHandle(x) => {
				todo!("Method handle loading not yet implemented");
				Value::NULL
			}
			ConstantPoolEntry::MethodType(x) => {
				todo!("Method type loading not yet implemented");
				Value::NULL
			}
			ConstantPoolEntry::Dynamic(x) => {
				todo!("Dynamic loading not yet implemented");
				Value::NULL
			}
			_ => {
				panic!(
					"Cannot load constant, is not of loadable type: {:?}. ",
					thing
				);
			}
		};
		self.push(resolved);
		Ok(ExecutionResult::Continue)
	}
}

pub fn string_from_bytes(byte_slice: &[jbyte]) -> String {
	// Convert the byte array (i8) to UTF-16 code units
	// The bytes are stored as UTF-16 LE (little-endian) pairs
	let bytes: Vec<u8> = byte_slice.iter().map(|&b| b as u8).collect();

	// Convert pairs of bytes to u16 (UTF-16 code units)
	let mut utf16_chars = Vec::new();
	for chunk in bytes.chunks_exact(2) {
		let code_unit = u16::from_le_bytes([chunk[0], chunk[1]]);
		utf16_chars.push(code_unit);
	}

	// Convert UTF-16 to Rust String (UTF-8)
	let rust_string = String::from_utf16_lossy(&utf16_chars);
	rust_string
}