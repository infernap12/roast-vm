use crate::objects::object::{Reference, ReferenceKind};
use crate::prim::Primitive;
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
use std::sync::{Arc, Mutex};
use std::ops::{Deref, DerefMut};
use crate::class::RuntimeClass;
use crate::error::VmError;

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

	pub fn class(&self) -> Arc<RuntimeClass> {
		match self {
			ArrayReference::Int(x) => x.lock().unwrap().class.clone(),
			ArrayReference::Byte(x) => x.lock().unwrap().class.clone(),
			ArrayReference::Short(x) => x.lock().unwrap().class.clone(),
			ArrayReference::Long(x) => x.lock().unwrap().class.clone(),
			ArrayReference::Float(x) => x.lock().unwrap().class.clone(),
			ArrayReference::Double(x) => x.lock().unwrap().class.clone(),
			ArrayReference::Char(x) => x.lock().unwrap().class.clone(),
			ArrayReference::Boolean(x) => x.lock().unwrap().class.clone(),
			ArrayReference::Object(x) => x.lock().unwrap().class.clone(),
		}
	}
}

pub type PrimitiveArrayReference<T: Primitive> = Arc<Mutex<Array<T>>>;

pub type ObjectArrayReference = Arc<Mutex<Array<Option<ReferenceKind>>>>;

#[derive(Debug)]
pub struct Array<T: ArrayValue> {
	pub(crate) id: u32,
	pub(crate) class: Arc<RuntimeClass>,
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

	pub fn new(id: u32, class: Arc<RuntimeClass>, length: i32) -> Self {
		Self {
			id,
			class,
			backing: vec![T::default(); length as usize].into_boxed_slice(),
		}
	}

	pub fn len(&self) -> jint {
		self.backing.len() as jint
	}
}

impl<T: Primitive + ArrayValue> From<(u32, Arc<RuntimeClass>, Vec<T>)> for Array<T> {
	fn from(value: (u32, Arc<RuntimeClass>, Vec<T>)) -> Self {
		let (id, class, vector) = value;
		Self {
			id,
			class,
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


impl ArrayReference {
	pub fn copy_from(
		&self,
		src: &ArrayReference,
		src_pos: jint,
		dst_pos: jint,
		length: jint,
	) -> Result<(), VmError> {
		macro_rules! copy {
            ($src_arr:expr, $dst_arr:expr) => {{
                let src_guard = $src_arr.lock().unwrap();
                let mut dst_guard = $dst_arr.lock().unwrap();

                let src_start = src_pos as usize;
                let dst_start = dst_pos as usize;
                let len = length as usize;

                // Bounds check
                if src_pos < 0 || dst_pos < 0 || length < 0
                    || src_start + len > src_guard.backing.len()
                    || dst_start + len > dst_guard.backing.len()
                {
                    return Err(VmError::InvariantError("Index oob".to_string()));
                }

                if Arc::ptr_eq($src_arr, $dst_arr) {
                    drop(src_guard);
                    dst_guard.backing.copy_within(src_start..src_start + len, dst_start);
                } else {
                    dst_guard.backing[dst_start..dst_start + len]
                        .copy_from_slice(&src_guard.backing[src_start..src_start + len]);
                }
                Ok(())
            }};
        }

		use ArrayReference::*;
		match (src, self) {
			(Int(s), Int(d)) => copy!(s, d),
			(Byte(s), Byte(d)) => copy!(s, d),
			(Short(s), Short(d)) => copy!(s, d),
			(Long(s), Long(d)) => copy!(s, d),
			(Float(s), Float(d)) => copy!(s, d),
			(Double(s), Double(d)) => copy!(s, d),
			(Char(s), Char(d)) => copy!(s, d),
			(Boolean(s), Boolean(d)) => copy!(s, d),
			(Object(s), Object(d)) => {
				// Object arrays need clone, not copy
				let src_guard = s.lock().unwrap();
				let mut dst_guard = d.lock().unwrap();

				let src_start = src_pos as usize;
				let dst_start = dst_pos as usize;
				let len = length as usize;

				if src_pos < 0 || dst_pos < 0 || length < 0
					|| src_start + len > src_guard.backing.len()
					|| dst_start + len > dst_guard.backing.len()
				{
					return Err(VmError::InvariantError("Index oob".to_string()));
				}

				if Arc::ptr_eq(s, d) {
					drop(src_guard);
					for i in if src_start < dst_start { (0..len).rev().collect::<Vec<_>>() } else { (0..len).collect() } {
						dst_guard.backing[dst_start + i] = dst_guard.backing[src_start + i].clone();
					}
				} else {
					for i in 0..len {
						dst_guard.backing[dst_start + i] = src_guard.backing[src_start + i].clone();
					}
				}
				Ok(())
			}
			_ => Err(VmError::InvariantError("Array type mismatch".to_string())),
		}
	}
}