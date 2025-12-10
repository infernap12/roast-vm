use crate::class::RuntimeClass;
use crate::objects::array::{ArrayReference, ObjectArrayReference, PrimitiveArrayReference};
use crate::prim::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
use crate::value::Value;
use dashmap::DashMap;
use log::trace;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use crate::class_file::FieldRef;
use crate::{string_from_bytes, BaseType, FieldType};

pub type ObjectReference = Arc<Mutex<Object>>;

#[derive(Debug, Clone)]
pub struct Object {
	pub id: u32,
	pub class: Arc<RuntimeClass>,
	pub fields: DashMap<String, Value>,
}

impl Object {
	pub fn set_field(&self, field_name: &str, value: Value) {
		trace!("Fields for object:\n\t{:#}", self.format_fields());
		trace!("Setting '{}' to '{}'", field_name, value);
		self.fields.insert(field_name.to_string(), value);
	}

	pub fn get_field(&self, field_ref: &FieldRef) -> Value {
		trace!("Fields for object:\n\t{:#}", self.format_fields());
		self.fields
			.get(&field_ref.name)
			.map(|e| e.clone())
			.unwrap_or_else(||{
				let initial = match &field_ref.desc {
					FieldType::Base(base) => {
						match base {
							BaseType::Byte => {
								Value::from(0i8)
							}
							BaseType::Char => { Value::from(0u16) }
							BaseType::Double => { Value::from(0f64) }
							BaseType::Float => { Value::from(0f32) }
							BaseType::Int => { Value::from(0i32) }
							BaseType::Long => { Value::from(0i64) }
							BaseType::Short => { Value::from(0i16) }
							BaseType::Boolean => { Value::from(false) }
						}
					}
					FieldType::ClassType(_) => { Value::NULL }
					FieldType::ArrayType(_) => { Value::NULL }
				};
				self.fields.insert(field_ref.name.clone(), initial.clone());
				initial
			})
	}

	fn format_fields(&self) -> ObjectFields<'_, String, Value> {
		ObjectFields(&self.fields)
	}
}

impl Display for Object {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.class.this_class == "java/lang/String" {}
		write!(f, "Object[id={}, class={}]", self.id, self.class.this_class)
	}
}
struct ObjectFields<'a, K, V>(&'a DashMap<K, V>);

impl<K, V> Display for ObjectFields<'_, K, V>
where
	K: Display + Eq + Hash + std::fmt::Debug,
	V: Display,
{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let mut debug_map = f.debug_map();
		for r in self.0 {
			let (k, v) = r.pair();
			debug_map.entry(k, &format!("{v}"));
		}
		debug_map.finish()
	}
}

impl PartialEq for Object {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for Object {}

#[derive(Debug, Clone)]
pub enum ReferenceKind {
	ObjectReference(ObjectReference),
	ArrayReference(ArrayReference),
}

impl ReferenceKind {
	pub fn into_object_reference(self) -> Option<ObjectReference> {
		match self {
			Self::ObjectReference(inner) => Some(inner),
			_ => None,
		}
	}
	pub fn id(&self) -> u32 {
		match self {
			Self::ObjectReference(r) => r.lock().unwrap().id,
			Self::ArrayReference(a) => a.id(),
		}
	}
	pub fn class(&self) -> Arc<RuntimeClass> {
		match self {
			Self::ObjectReference(r) => r.lock().unwrap().class.clone(),
			Self::ArrayReference(a) => a.class(),
		}
	}
}

pub type Reference = Option<ReferenceKind>;

impl Display for ReferenceKind {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let id = match self {
			ReferenceKind::ObjectReference(x) => {
				let guard = x.lock().unwrap();
				if guard.class.this_class == "java/lang/String"
					&& let Some(field) = guard.fields.get("value")
					&& let Value::Reference(Some(ReferenceKind::ArrayReference(ArrayReference::Byte(actual)))) = field.value()
				{
					let arr_guard= actual.lock().unwrap();
					let string = crate::string_from_bytes(&arr_guard);
					format!("\u{AB}{}\u{BB}", string)
				} else {
					format!("Obj<{}>", guard.class.this_class)
				}
			}
			ReferenceKind::ArrayReference(ArrayReference::Int(x)) => {
				let guard = x.lock().unwrap();
				format!("int{:?}", guard.backing)
			}
			ReferenceKind::ArrayReference(ArrayReference::Byte(x)) => {
				let guard = x.lock().unwrap();
				format!("byte{:?}", guard.backing)
			}
			ReferenceKind::ArrayReference(ArrayReference::Short(x)) => {
				let guard = x.lock().unwrap();
				format!("short{:?}", guard.backing)
			}
			ReferenceKind::ArrayReference(ArrayReference::Long(x)) => {
				let guard = x.lock().unwrap();
				format!("long{:?}", guard.backing)
			}
			ReferenceKind::ArrayReference(ArrayReference::Float(x)) => {
				let guard = x.lock().unwrap();
				format!("float{:?}", guard.backing)
			}
			ReferenceKind::ArrayReference(ArrayReference::Double(x)) => {
				let guard = x.lock().unwrap();
				format!("double{:?}", guard.backing)
			}
			ReferenceKind::ArrayReference(ArrayReference::Char(x)) => {
				let guard = x.lock().unwrap();
				format!("char{:?}", guard.backing)
			}
			ReferenceKind::ArrayReference(ArrayReference::Boolean(x)) => {
				let guard = x.lock().unwrap();
				format!("boolean{:?}", guard.backing)
			}
			ReferenceKind::ArrayReference(ArrayReference::Object(x)) => {
				let guard = x.lock().unwrap();
				format!("object[{:?}]", guard.id)
			}
		};
		write!(f, "{}", id)
	}
}

impl From<ObjectReference> for ReferenceKind {
	fn from(value: ObjectReference) -> Self {
		Self::ObjectReference(value)
	}
}

impl From<PrimitiveArrayReference<jint>> for ReferenceKind {
	fn from(value: PrimitiveArrayReference<jint>) -> Self {
		Self::ArrayReference(ArrayReference::Int(value))
	}
}

impl From<PrimitiveArrayReference<jbyte>> for ReferenceKind {
	fn from(value: PrimitiveArrayReference<jbyte>) -> Self {
		Self::ArrayReference(ArrayReference::Byte(value))
	}
}

impl From<PrimitiveArrayReference<jshort>> for ReferenceKind {
	fn from(value: PrimitiveArrayReference<jshort>) -> Self {
		Self::ArrayReference(ArrayReference::Short(value))
	}
}

impl From<PrimitiveArrayReference<jlong>> for ReferenceKind {
	fn from(value: PrimitiveArrayReference<jlong>) -> Self {
		Self::ArrayReference(ArrayReference::Long(value))
	}
}

impl From<PrimitiveArrayReference<jfloat>> for ReferenceKind {
	fn from(value: PrimitiveArrayReference<jfloat>) -> Self {
		Self::ArrayReference(ArrayReference::Float(value))
	}
}

impl From<PrimitiveArrayReference<jdouble>> for ReferenceKind {
	fn from(value: PrimitiveArrayReference<jdouble>) -> Self {
		Self::ArrayReference(ArrayReference::Double(value))
	}
}

impl From<PrimitiveArrayReference<jchar>> for ReferenceKind {
	fn from(value: PrimitiveArrayReference<jchar>) -> Self {
		Self::ArrayReference(ArrayReference::Char(value))
	}
}

impl From<PrimitiveArrayReference<jboolean>> for ReferenceKind {
	fn from(value: PrimitiveArrayReference<jboolean>) -> Self {
		Self::ArrayReference(ArrayReference::Boolean(value))
	}
}

impl From<ObjectArrayReference> for ReferenceKind {
	fn from(value: ObjectArrayReference) -> Self {
		Self::ArrayReference(ArrayReference::Object(value))
	}
}
