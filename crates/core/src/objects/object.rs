use crate::class::RuntimeClass;
use crate::value::Value;
use dashmap::DashMap;
use log::trace;
use std::fmt::Display;
use std::sync::{Arc, Mutex};

pub type ObjectReference = Arc<Mutex<Object>>;

#[derive(Debug, Clone)]
pub struct Object {
	pub id: u32,
	pub class: Arc<RuntimeClass>,
	pub fields: DashMap<String, Value>,
}

impl Object {
	pub fn set_field(&self, field_name: &str, value: Value) {
		trace!("Fields for object:\n\t{:?}", self.fields);
		trace!("Setting '{}' to '{}'", field_name, value);
		self.fields.insert(field_name.to_string(), value);
	}

	pub fn get_field(&self, field_name: &str) -> Value {
		trace!("Fields for object:\n\t{:?}", self.fields);
		self.fields
			.get(&field_name.to_string())
			.map(|e| e.clone())
			.unwrap_or(Value::NULL)
	}
}

impl Display for Object {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Object[id={}, class={}]", self.id, self.class.this_class)
	}
}

impl PartialEq for Object {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for Object {}
