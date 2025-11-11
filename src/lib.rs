use std::cell::RefCell;
use std::io::Read;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use deku::DekuContainerRead;
use deku_derive::{DekuRead, DekuWrite};
use itertools::Itertools;
use vm::Vm;
use crate::attributes::{Attribute, CodeAttribute, Ops};
use crate::class_file::{Bytecode, ClassFile, ClassFlags, ConstantPoolExt, CpInfo, FieldFlags, MethodFlags};
use crate::object::Object;
use crate::thread::VmThread;

mod attributes;
mod class_file;
mod class_loader;
mod macros;
mod bimage;
mod vm;
mod object;
mod rng;
mod thread;
mod class;

const NULL: Value = Value::Reference(None);

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
pub fn run() {
	env_logger::init();
	// let mut cl = ClassLoader::new().unwrap();
	// cl.load_class("org.example.App").expect("TODO: panic message");
	// let clazz = cl.get_or_load("org.example.App").unwrap();
	// for (i, (k, v)) in cl.classes().iter().enumerate() {
	//     std::fs::write(format!("./output/{}-{}.txt", i, class_loader::path_to_dot(k)), format!("{}\n{}", k, v)).unwrap();
	// }

	let mut class_file = File::open("./data/org/example/App.class").unwrap();
	let mut bytes = Vec::new();
	class_file.read_to_end(&mut bytes).unwrap();
	let (_rest, clazz) = ClassFile::from_bytes((bytes.as_ref(), 0)).unwrap();
	let method = clazz.methods.iter().nth(1).unwrap().clone();
	let code = method.attributes.iter().find_map(|x| {
		if let Some(Attribute::Code(code_attr)) = &x.get(&clazz) {
			Some(code_attr.clone())
		} else {
			None
		}
	}).unwrap();
	// let frame = Frame::new();
	// println!("{}", code);
	let mut buf = Vec::new();
	let bytes = code.code_length.to_be_bytes();
	buf.extend_from_slice(&bytes);
	buf.extend_from_slice(&code.code.clone());
	let (_rest, ops) = Bytecode::from_bytes((buf.as_ref(), 0)).unwrap();
	let var_table = code.attributes.iter().find_map(|x| {
		if let Some(Attribute::LocalVariableTable(varTableAttr)) = &x.get(&clazz) {
			Some(varTableAttr.clone())
		} else {
			None
		}
	}).unwrap();
	println!("{}", clazz);
	let pool = clazz.constant_pool;
	let mut vm = Arc::new(Vm::new());
	let mut frame = Frame::new(code, pool, Default::default(), vm);
	// println!("{:?}", frame);
	frame.execute();

	// println!("{:?}", ops);
	// println!("{:?}", var_table.local_variable_table);
	// vm.method(ops.clone(), code, var_table);
}



#[derive(Debug)]
pub struct KlassField {
	pub name: String,
	pub field_flags: FieldFlags,
	pub descriptor: FieldType,
}
#[derive(Debug)]
pub struct KlassMethod {
	pub name: String,
	pub method_flags: MethodFlags,
	pub method_descriptor: MethodDescriptor,
	pub code_attribute: CodeAttribute
}

type ObjectRef = Arc<Mutex<Object>>;
#[derive(Debug)]
#[derive(Clone)]
enum Value {
	Boolean(bool),
	Char(char),
	Float(f32),
	Double(f64),
	Byte(i8),
	Short(i16),
	Int(i32),
	Long(i64),
	Reference(Option<ObjectRef>),
}


struct Frame {

	// program counter
	pc: u16,
	// operand stack
	stack: Vec<Value>,
	// local vars
	vars: Vec<Value>,
	// constant pool
	pool: Arc<Vec<CpInfo>>,

	bytecode: Bytecode,

	thread: Arc<VmThread>
}

impl Display for Frame {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "PC: {}\nStack: {:?}\nVars: {:?}", self.pc, self.stack, self.vars)
	}
}

//  println!("State:\n\tStack: {:?}\n\tLocals :{:?}\n", self.stack, self.vars) }


impl Frame {
	fn load_constant(index: u8) {}
	fn new(code_attr: CodeAttribute, pool: Arc<Vec<CpInfo>>, mut locals: Vec<Value>, thread: Arc<VmThread>) -> Self {
		let max_stack = code_attr.max_stack as usize;
		let max_local = code_attr.max_locals as usize;
		let bytes = code_attr.code_length.to_be_bytes();
		let mut buf = Vec::new();
		buf.extend_from_slice(&bytes);
		buf.extend_from_slice(&code_attr.code.clone());
		let (_rest, bytecode) = Bytecode::from_bytes((buf.as_ref(), 0)).unwrap();
		let extend = vec![Value::Reference(None); max_local-locals.len()];
		locals.extend_from_slice(&extend);
		Frame {
			pc: 0,
			stack: Vec::with_capacity(max_stack),
			vars: locals,
			pool,
			bytecode,
			vm,
		}
	}
	fn execute(&mut self) {
		let binding = self.bytecode.code.clone();
		let mut ops = binding.iter();
		while let Some(op) = ops.next() {
			println!("Executing Op: {:?}", op);
			let result = self.execute_instruction(op);
			match result {
				Ok(_) => { println!("State:\n\tStack: {:?}\n\tLocals :{:?}\n", self.stack, self.vars) }
				Err(_) => {panic!("Mission failed, we'll get em next time")}
			}


		}
		()
	}
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
#[deku(seek_from_current = "-1")]
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
			_ => panic!("Invalid base type: {}", value)
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct MethodDescriptor {
	parameters: Vec<FieldType>,
	// none = void/v
	return_type: Option<FieldType>
}
#[derive(Debug, PartialEq)]
pub enum FieldType {
	Base(BaseType),
	// L ClassName
	ClassType(String),
	// [ ComponentType
	ArrayType(Box<FieldType>),
}

enum ExecutionResult {
	Continue,
	Return(()),
	ReturnValue(Value)
}
impl Frame {
	fn execute_instruction(&mut self, op: &Ops) -> Result<ExecutionResult, ()> {
		match op {
			Ops::ldc(index) => {
				let thing = self.pool.get_constant(index.to_owned() as u16).ok_or(())?;
				println!("\tLoading constant: {}", thing);
				let resolved: Option<Value> = match thing {
					CpInfo::Utf8(x) => { println!("{:?}", String::from_utf8(x.bytes.clone())); None }
					CpInfo::Integer(x) => { Some(Value::Int(x.clone())) }
					CpInfo::Float(x) => { Some(Value::Float(x.clone())) }
					// CpInfo::Long(x) => { None }
					// CpInfo::Double(x) => {None}
					CpInfo::Class(x) => {None}
					CpInfo::String(x) => {
						// self.vm.
						None
					}
					// CpInfo::FieldRef(x) => {}
					CpInfo::MethodRef(x) => {None}
					// CpInfo::InterfaceMethodRef(x) => {}
					// CpInfo::NameAndType(x) => {}
					CpInfo::MethodHandle(x) => {None}
					CpInfo::MethodType(x) => { None }
					// CpInfo::Dynamic(x) => {}
					// CpInfo::InvokeDynamic(x) => {}
					// CpInfo::Module(x) => {}
					// CpInfo::Package(x) => {}
					_ => { None }
				};
				if let Some(x) = resolved {
					self.stack.push(x);
				};
				Ok(ExecutionResult::Continue)
			},
			Ops::ldc2_w(index) => {
				let val = self.pool.get_constant(*index).ok_or(())?;
				println!("\tLoading constant: {}", val);
				let resolved = match val {
					CpInfo::Double(x) => { Some(Value::Double(x.clone())) }
					CpInfo::Long(x) => { Some(Value::Long(x.clone())) }
					_ => { None }
				};
				if let Some(x) = resolved {
					self.stack.push(x);
				};
				Ok(ExecutionResult::Continue)
			}
			// store
			Ops::fstore(index) => { store!(self, f, *index as usize) },
			Ops::fstore_0 => { store!(self, f, 0) },
			Ops::fstore_1 => { store!(self, f, 1) },
			Ops::fstore_2 => { store!(self, f, 2) },
			Ops::fstore_3 => { store!(self, f, 3) }
			Ops::dstore(index) => { store!(self, d, *index as usize) },
			Ops::dstore_0 => { store!(self, d, 0) },
			Ops::dstore_1 => { store!(self, d, 1) },
			Ops::dstore_2 => { store!(self, d, 2) },
			Ops::dstore_3 => { store!(self, d, 3) }

			// load
			Ops::fload(index) => { load!(self, f, *index as usize) }
			Ops::fload_0 => { load!(self, f, 0) }
			Ops::fload_1 => { load!(self, f, 1) }
			Ops::fload_2 => { load!(self, f, 2) }
			Ops::fload_3 => { load!(self, f, 3) }
			Ops::dload(index) => { load!(self, d, *index as usize) }
			Ops::dload_0 => { load!(self, d, 0) }
			Ops::dload_1 => { load!(self, d, 1) }
			Ops::dload_2 => { load!(self, d, 2) }
			Ops::dload_3 => { load!(self, d, 3) }

			Ops::f2d => {
				if let Value::Float(float) = self.stack.pop().expect("Stack must have value") {
					let double: f64 = float.into();
					self.stack.push(Value::Double(double));
					Ok(ExecutionResult::Continue)
				} else { Err(()) }
			}

			Ops::dadd => {
				let value1 = self.stack.pop().expect("Stack must have value");
				let value2 = self.stack.pop().expect("Stack must have value");
				if let (Value::Double(value1), Value::Double(value2)) = (value1, value2) {
					self.stack.push(Value::Double(value1 + value2));
					Ok(ExecutionResult::Continue)
				} else { Err(()) }
			}

			// get static field
			Ops::getstatic(index) => {
				let field = self.pool.resolve_field(*index)?;
				// let (code, pool) = {
				// 	let mut loader = self.vm.loader.lock().unwrap();
				// 	let class = loader.get_or_load(&field.class).unwrap();
				// 	let field = class.get_static_field_value(&field)
				// 		// let code = class.get_code(meth)?;
				// 		(code, pool)
				// };
				println!("{:?}", field);
				todo!("Finish get static");
				Ok(ExecutionResult::Continue)
			},

			Ops::invokevirtual(index) => {
				let meth = self.pool.resolve_method_ref(*index)?;
				let params = meth.desc.num_arguments();
				let last = self.stack.len() - 1;
				let first = last - params + 1;
				let slice = self.stack.get(first..last).unwrap().to_vec();
				//sub slice param length + one, throw it to frame new
				let (code, pool) = {
					let mut loader = self.vm.loader.lock().unwrap();
					let class = loader.get_or_load(&meth.class).unwrap();
					let pool = class.constant_pool.clone();
					let code = class.get_code(meth)?;
					(code, pool)
				};
				// let code = class.get_code(meth)?;
				// let class = self.vm.loader.get_or_load(&meth.class).unwrap();
				// let pool = &class.constant_pool;
				let vars = slice;
				let frame = Frame::new(code, pool.clone(), vars, self.vm.clone());
				// println!("{:?}", meth);
				// todo!("Finish invoke virtual");
				Ok(ExecutionResult::Continue)
			}

			Ops::return_void => {
				Ok(ExecutionResult::Return(()))
			}
			_ => { todo!("Unimplemented op: {:?}", op) }
		}
	}
}