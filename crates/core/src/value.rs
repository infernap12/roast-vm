use crate::objects::array::ArrayReference;
use crate::objects::object::{ObjectReference, ReferenceKind};
use crate::{BaseType, FieldType, VmError};
use core::fmt;
use dashmap::DashMap;
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
use log::trace;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

/// A reference-counted, thread-safe pointer to an Object.

/// Represents a JVM runtime value.
///
/// This enum covers all primitive types and object references that can exist
/// on the operand stack or in local variables during bytecode execution.
#[derive(Debug, Clone)]
pub enum Value {
	Primitive(Primitive),
	/// Reference to an object (or null)
	Reference(Option<ReferenceKind>),
	/// Second slot of a wide value (long/double). Should never be accessed directly.
	Padding,
}

#[derive( Clone, Default)]
pub struct OperandStack(Vec<Value>);

impl OperandStack {
	pub fn new() -> Self {
		Self(Vec::new())
	}
	pub fn with_capacity(max_stack: usize) -> Self {
		let backing = Vec::with_capacity(max_stack);
		Self {
			0: backing
		}
	}

	pub fn push(&mut self, value: Value) {
		self.0.push(value);
	}

	pub fn pop(&mut self) -> Result<Value, VmError> {
		self.0.pop()
			.ok_or_else(|| VmError::StackError("Stack underflow".to_string()))
	}

	pub fn peek(&self) -> Result<&Value, VmError> {
		self.0.last()
			.ok_or_else(|| VmError::StackError("Stack underflow on peek".to_string()))
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	/// Pop n values, returned in the order they were pushed (not pop order)
	pub fn pop_n(&mut self, n: usize) -> Result<Vec<Value>, VmError> {
		let start = self.0.len()
			.checked_sub(n)
			.ok_or_else(|| VmError::StackError("Stack underflow on pop_n".to_string()))?;
		Ok(self.0.drain(start..).collect())
	}
}
impl std::fmt::Debug for OperandStack {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.0.iter()).finish()
	}
}

impl OperandStack {
	pub fn iter(&self) -> std::slice::Iter<'_, Value> {
		self.into_iter()
	}
}

impl<'a> IntoIterator for &'a OperandStack {
	type Item = &'a Value;
	type IntoIter = std::slice::Iter<'a, Value>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.iter()
	}
}
#[derive(Clone, Default)]
pub struct LocalVariables {
	inner: Vec<Value>
}

impl LocalVariables {
	pub fn with_capacity(max_locals: usize) -> Self {
		Self {
			inner: vec![Value::NULL; max_locals]
		}
	}
	pub fn from_args(args: Vec<Value>, max_locals: usize) -> Self {
		let mut locals = Self::with_capacity(max_locals);
		let mut idx = 0;
		for arg in args {
			locals.set(idx, arg.clone());
			idx += if arg.is_wide() { 2 } else { 1 };
		}
		locals
	}

	pub fn set(&mut self, index: usize, value: Value) {
		let idx = index;
		let is_wide = value.is_wide();
		self.inner[idx] = value;
		if is_wide {
			self.inner[idx + 1] = Value::Padding;
		}
	}

	pub fn get(&self, index: usize) -> &Value {
		let val = &self.inner[index];
		if matches!(val, Value::Padding) {
			panic!(
				"Attempted to read padding slot at index {} (second half of wide value at {})",
				index,
				index - 1
			);
		}
		val
	}

	pub fn iter(&self) -> impl Iterator<Item=&Value> {
		self.inner.iter().filter(|v| !matches!(v, Value::Padding))
	}

	pub fn slots(&self) -> impl Iterator<Item=(u16, &Value)> {
		self.inner
			.iter()
			.enumerate()
			.filter(|(_, v)| !matches!(v, Value::Padding))
			.map(|(i, v)| (i as u16, v))
	}
}

impl std::fmt::Debug for LocalVariables {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list()
			.entries(self.slots().map(|(i, v)| format!("[{}]: {:?}", i, v)))
			.finish()
	}
}


impl Value {
	pub const NULL: Value = Value::Reference(None);

	pub fn format_vec(values: &Vec<Value>) -> String {
		let fmt = values
			.iter()
			.map(|v| v.to_string())
			.collect::<Vec<String>>()
			.join(", ");

		format!("[{}]", fmt)
	}

	pub fn is_wide(&self) -> bool {
		matches!(self, Value::Primitive(Primitive::Long(_) | Primitive::Double(_)))
	}
}

macro_rules! impl_value_from {
	($type:ty, $variant:ident) => {
		impl From<$type> for Value {
			fn from(value: $type) -> Self {
				Self::Primitive(Primitive::$variant(value))
			}
		}
	};
}
impl_value_from!(jint, Int);
impl_value_from!(jlong, Long);
impl_value_from!(jfloat, Float);
impl_value_from!(jdouble, Double);
impl_value_from!(jbyte, Byte);
impl_value_from!(jshort, Short);
impl_value_from!(jchar, Char);

impl From<jboolean> for Value {
	fn from(value: jboolean) -> Self {
		Self::Primitive(Primitive::Boolean(value != 0))
	}
}

impl_value_from!(bool, Boolean);

impl From<Option<ReferenceKind>> for Value {
	fn from(value: Option<ReferenceKind>) -> Self {
		Self::Reference(value)
	}
}

impl From<ReferenceKind> for Value {
	fn from(value: ReferenceKind) -> Self {
		Value::from(Some(value))
	}
}
impl From<ObjectReference> for Value {
	fn from(value: ObjectReference) -> Self {
		Value::from(ReferenceKind::ObjectReference(value))
	}
}
impl From<ArrayReference> for Value {
	fn from(value: ArrayReference) -> Self {
		Value::from(ReferenceKind::ArrayReference(value))
	}
}

#[derive(Debug, Clone)]
pub enum Primitive {
	/// Boolean value (true/false)
	Boolean(bool),
	/// Unicode character
	Char(u16),
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
}

impl Display for Primitive {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Primitive::Boolean(b) => write!(f, "bool({})", b),
			Primitive::Char(c) => {
				let ch = char::from_u32(*c as u32)
					.map(|c| format!("'{}'", c))
					.unwrap_or_else(|| format!("'\\u{{{:04x}}}'", c));
				write!(f, "{}", ch)
			}
			Primitive::Float(fl) => write!(f, "float({})", fl),
			Primitive::Double(d) => write!(f, "double({})", d),
			Primitive::Byte(b) => write!(f, "byte({})", b),
			Primitive::Short(s) => write!(f, "short({})", s),
			Primitive::Int(i) => write!(f, "int({})", i),
			Primitive::Long(l) => write!(f, "long({})", l),
		}
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Primitive(prim) => write!(f, "{}", prim),
			Value::Reference(Some(obj)) => write!(f, "Ref({})", obj),
			Value::Reference(None) => write!(f, "null"),
			_ => { write!(f, "pad") }
		}
	}
}

impl PartialEq<FieldType> for Value {
	fn eq(&self, other: &FieldType) -> bool {
		match self {
			Value::Primitive(prim) => match prim {
				Primitive::Boolean(_) => other.eq(&FieldType::Base(BaseType::Boolean)),
				Primitive::Char(_) => other.eq(&FieldType::Base(BaseType::Char)),
				Primitive::Float(_) => other.eq(&FieldType::Base(BaseType::Float)),
				Primitive::Double(_) => other.eq(&FieldType::Base(BaseType::Double)),
				Primitive::Byte(_) => other.eq(&FieldType::Base(BaseType::Byte)),
				Primitive::Short(_) => other.eq(&FieldType::Base(BaseType::Short)),
				Primitive::Int(_) => {
					trace!("{other:?}");
					trace!("{self}");
					let result = match other {
						FieldType::Base(BaseType::Boolean)
						| FieldType::Base(BaseType::Byte)
						| FieldType::Base(BaseType::Char)
						| FieldType::Base(BaseType::Short)
						| FieldType::Base(BaseType::Int) => true,
						_ => false,
					};
					trace!("{}", result);
					result
				}
				Primitive::Long(_) => other.eq(&FieldType::Base(BaseType::Long)),
			},
			Value::Reference(refe) => match refe {
				None => match other {
					FieldType::Base(_) => false,
					FieldType::ClassType(_) => true,
					FieldType::ArrayType(_) => true,
				},
				Some(kind) => match kind {
					ReferenceKind::ObjectReference(found_ref) => {
						if let FieldType::ClassType(expected) = other {
							let found = format!("L{};", found_ref.lock().unwrap().class.this_class);
							expected.eq(&found)
						} else {
							false
						}
					}
					ReferenceKind::ArrayReference(x) => match x {
						ArrayReference::Int(x) => {
							matches!(other, FieldType::ArrayType(inner) if matches!(**inner, FieldType::Base(BaseType::Int)))
						}
						ArrayReference::Byte(x) => {
							matches!(other, FieldType::ArrayType(inner) if matches!(**inner, FieldType::Base(BaseType::Byte)))
						}
						ArrayReference::Short(x) => {
							matches!(other, FieldType::ArrayType(inner) if matches!(**inner, FieldType::Base(BaseType::Short)))
						}
						ArrayReference::Long(x) => {
							matches!(other, FieldType::ArrayType(inner) if matches!(**inner, FieldType::Base(BaseType::Long)))
						}
						ArrayReference::Float(x) => {
							matches!(other, FieldType::ArrayType(inner) if matches!(**inner, FieldType::Base(BaseType::Float)))
						}
						ArrayReference::Double(x) => {
							matches!(other, FieldType::ArrayType(inner) if matches!(**inner, FieldType::Base(BaseType::Double)))
						}
						ArrayReference::Char(x) => {
							matches!(other, FieldType::ArrayType(inner) if matches!(**inner, FieldType::Base(BaseType::Char)))
						}
						ArrayReference::Boolean(x) => {
							matches!(other, FieldType::ArrayType(inner) if matches!(**inner, FieldType::Base(BaseType::Boolean)))
						}
						ArrayReference::Object(x) => {
							if let FieldType::ArrayType(inner) = other {
								if let FieldType::ClassType(_) = **inner {
									true
								} else {
									false
								}
							} else {
								false
							}
						}
					},
				},
			},
			_ => { panic!("uhh what") }
		}
	}
}
