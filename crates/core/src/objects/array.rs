use crate::objects::object::{Reference, ReferenceKind};
use crate::prim::Primitive;
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
use std::sync::{Arc, Mutex};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub enum ArrayReference {
	Int(PrimitiveArrayReference<jint>),
	Byte(PrimitiveArrayReference<jbyte>),
	Short(PrimitiveArrayReference<jshort>),
	Long(PrimitiveArrayReference<jlong>),
	Float(PrimitiveArrayReference<jfloat>),
	Double(PrimitiveArrayReference<jdouble>),
	Char(PrimitiveArrayReference<jchar>),
	Boolean(PrimitiveArrayReference<jboolean>),
	Object(ObjectArrayReference),
}

impl ArrayReference {
	pub fn len(&self) -> jint {
		match self {
			ArrayReference::Int(x) => x.lock().unwrap().len(),
			ArrayReference::Byte(x) => x.lock().unwrap().len(),
			ArrayReference::Short(x) => x.lock().unwrap().len(),
			ArrayReference::Long(x) => x.lock().unwrap().len(),
			ArrayReference::Float(x) => x.lock().unwrap().len(),
			ArrayReference::Double(x) => x.lock().unwrap().len(),
			ArrayReference::Char(x) => x.lock().unwrap().len(),
			ArrayReference::Boolean(x) => x.lock().unwrap().len(),
			ArrayReference::Object(x) => x.lock().unwrap().len(),
		}
	}

	pub fn id(&self) -> u32 {
		match self {
			ArrayReference::Int(x) => x.lock().unwrap().id,
			ArrayReference::Byte(x) => x.lock().unwrap().id,
			ArrayReference::Short(x) => x.lock().unwrap().id,
			ArrayReference::Long(x) => x.lock().unwrap().id,
			ArrayReference::Float(x) => x.lock().unwrap().id,
			ArrayReference::Double(x) => x.lock().unwrap().id,
			ArrayReference::Char(x) => x.lock().unwrap().id,
			ArrayReference::Boolean(x) => x.lock().unwrap().id,
			ArrayReference::Object(x) => x.lock().unwrap().id,
		}
	}
}

pub type PrimitiveArrayReference<T: Primitive> = Arc<Mutex<Array<T>>>;

pub type ObjectArrayReference = Arc<Mutex<Array<Option<ReferenceKind>>>>;

#[derive(Debug)]
pub struct Array<T: ArrayValue> {
	pub(crate) id: u32,
	pub(crate) backing: Box<[T]>,
}

impl<T> Array<T>
where
	T: ArrayValue + Clone + Default,
{
	pub fn set(&mut self, index: i32, value: T) {
		self.backing[index as usize] = value
	}

	pub fn get(&self, index: i32) -> T {
		self.backing[index as usize].clone()
	}

	pub fn new(id: u32, length: i32) -> Self {
		Self {
			id,
			backing: vec![T::default(); length as usize].into_boxed_slice(),
		}
	}

	pub fn len(&self) -> jint {
		self.backing.len() as jint
	}
}

impl<T: Primitive + ArrayValue> From<(u32, Vec<T>)> for Array<T> {
	fn from(value: (u32, Vec<T>)) -> Self {
		let (id, vector) = value;
		Self {
			id,
			backing: vector.into_boxed_slice(),
		}
	}
}

impl<T: ArrayValue> Deref for Array<T> {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		&self.backing
	}
}

impl<T: ArrayValue> DerefMut for Array<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.backing
	}
}

pub trait ArrayValue {}

impl ArrayValue for Reference {}

impl ArrayValue for jbyte {}
impl ArrayValue for jshort {}
impl ArrayValue for jint {}
impl ArrayValue for jlong {}
impl ArrayValue for jchar {}

impl ArrayValue for jfloat {}
impl ArrayValue for jdouble {}

impl ArrayValue for jboolean {}
