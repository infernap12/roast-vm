use crate::objects::array::Reference;
use crate::objects::object::ObjectReference;
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
use std::fmt::{Display, Formatter};

/// A reference-counted, thread-safe pointer to an Object.

/// Represents a JVM runtime value.
///
/// This enum covers all primitive types and object references that can exist
/// on the operand stack or in local variables during bytecode execution.
#[derive(Debug, Clone)]
pub enum Value {
	Primitive(Primitive),
	/// Reference to an object (or null)
	Reference(Option<Reference>),
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
			Primitive::Char(c) => write!(f, "char({})", char::from_u32(*c as u32).unwrap_or('?')),
			Primitive::Float(fl) => write!(f, "float({})", fl),
			Primitive::Double(d) => write!(f, "double({})", d),
			Primitive::Byte(b) => write!(f, "byte({})", b),
			Primitive::Short(s) => write!(f, "short({})", s),
			Primitive::Int(i) => write!(f, "int({})", i),
			Primitive::Long(l) => write!(f, "long({})", l),
		}
	}
}

impl Value {
	pub const NULL: Value = Value::Reference(None);
}

impl Display for Value {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Primitive(prim) => write!(f, "{}", prim),
			Value::Reference(Some(obj)) => write!(f, "Ref({})", obj),
			Value::Reference(None) => write!(f, "null"),
		}
	}
}
