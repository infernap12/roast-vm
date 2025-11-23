use crate::objects::object::{Object, ObjectReference};
use crate::value::{Primitive, Value};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum Reference {
	ObjectReference(ObjectReference),
	ArrayReference(ArrayReference),
}

impl Display for Reference {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let id = match self {
			Reference::ObjectReference(x) => x.lock().unwrap().id,
			Reference::ArrayReference(ArrayReference::Primitive(x)) => x.lock().unwrap().0,
			Reference::ArrayReference(ArrayReference::Object(x)) => x.lock().unwrap().0,
		};
		write!(f, "{}", id)
	}
}

impl From<ObjectReference> for Reference {
	fn from(value: ObjectReference) -> Self {
		Self::ObjectReference(value)
	}
}

impl From<PrimitiveArrayReference> for Reference {
	fn from(value: PrimitiveArrayReference) -> Self {
		Self::ArrayReference(ArrayReference::Primitive(value))
	}
}

impl From<ObjectArrayReference> for Reference {
	fn from(value: ObjectArrayReference) -> Self {
		Self::ArrayReference(ArrayReference::Object(value))
	}
}

#[derive(Debug, Clone)]
pub enum ArrayReference {
	Primitive(PrimitiveArrayReference),
	Object(ObjectArrayReference),
}

pub type PrimitiveArrayReference = Arc<Mutex<PrimitiveArray>>;
type PrimitiveArray = (u32, Vec<Primitive>);

pub type ObjectArrayReference = Arc<Mutex<ObjectArray>>;
type ObjectArray = (u32, Vec<ObjectReference>);

#[derive(Debug)]
pub struct Array<T> {
	id: u32,
	backing: Vec<T>,
}

impl Array<Primitive> {
	fn set(&self, index: i32, value: Value) {
		let thing : [100, u32]
	}
}
