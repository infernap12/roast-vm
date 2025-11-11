use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use crate::class_file::ClassFile;
use crate::Value;

#[derive(Debug, Clone)]
pub struct Object {
	pub id: String,
	pub class: Arc<ClassFile>,
	fields: Arc<Mutex<Vec<Value>>>,
}

impl PartialEq for Object {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for Object {}