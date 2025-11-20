use crate::attributes::AttributeInfo;
use crate::class_file::{
	ClassFile, ClassFlags, ConstantPoolEntry, FieldData, FieldInfo, FieldRef, MethodData,
	MethodInfo, MethodRef,
};
use crate::{FieldType, MethodDescriptor, VmError};
use log::trace;
use std::sync::{Arc, Mutex};
use std::thread::ThreadId;

/// JVM Spec 5.5: Initialization states for a class
#[derive(Debug, Clone, PartialEq)]
pub enum InitState {
	/// Verified and prepared but not initialized
	NotInitialized,
	/// Being initialized by a specific thread
	Initializing(ThreadId),
	/// Fully initialized and ready for use
	Initialized,
	/// Initialization failed
	Error(String),
}

#[derive(Debug)]
pub struct RuntimeClass {
	pub constant_pool: Arc<Vec<ConstantPoolEntry>>,
	pub access_flags: ClassFlags,
	pub this_class: String,
	pub super_class: Option<Arc<RuntimeClass>>,
	pub interfaces: Vec<Arc<RuntimeClass>>,
	pub fields: Vec<FieldData>,
	pub methods: Vec<MethodData>,
	/// Thread-safe initialization state (JVM Spec 5.5)
	pub init_state: Mutex<InitState>,
}

impl RuntimeClass {
	pub fn find_method(&self, name: &str, desc: &MethodDescriptor) -> Result<&MethodData, VmError> {
		trace!(
			"Finding in {}, method Name: {name} desc:{desc},",
			self.this_class
		);
		if let Some(method) = self.methods.iter().find(|e| {
			let name_match = e.name.eq(name);
			let param_match = desc.parameters == e.desc.parameters;
			name_match && param_match
		}) {
			trace!("Found method: {name}");
			return Ok(method);
		};

		// recurse super class
		if let Some(super_class) = &self.super_class {
			trace!("Recursing for {name}");
			return super_class.find_method(name, desc);
		}
		// No method found, and we must be Object, as we don't have a superclass
		Err(VmError::LoaderError("Failed to find method".to_string()))
	}

	pub fn find_field(&self, name: &str, desc: FieldType) -> Result<&FieldData, VmError> {
		println!("Finding field");
		if let Some(field) = self.fields.iter().find(|e| {
			println!("Field Name Needed: {name}, Checked:{}", e.name);
			println!("Field type Needed: {desc:?}, Checked:{:?}", e.desc);
			let name_match = e.name.eq(name);
			let type_match = desc == e.desc;
			name_match && type_match
		}) {
			return Ok(field);
		};

		// recurse super class
		if let Some(super_class) = &self.super_class {
			return super_class.find_field(name, desc);
		}
		// No field found, and we must be Object, as we don't have a superclass
		Err(VmError::LoaderError("Failed to find field".to_string()))
	}
}
