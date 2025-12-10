use crate::attributes::AttributeInfo;
use crate::class_file::{
	ClassFile, ClassFlags, ConstantPoolEntry, FieldData, FieldInfo, FieldRef, MethodData,
	MethodInfo, MethodRef,
};
use crate::{FieldType, MethodDescriptor};
use log::trace;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, OnceLock, OnceState};
use std::thread::ThreadId;
use crate::error::VmError;

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
	pub mirror: OnceLock<u32>,
	/// Thread-safe initialization state (JVM Spec 5.5)
	pub init_state: Mutex<InitState>,
	pub super_classes: Vec<Arc<RuntimeClass>>,
	pub super_interfaces: Vec<Arc<RuntimeClass>>,
	pub component_type: Option<Arc<RuntimeClass>>,
	pub source_file: Option<String>,
}
impl Hash for RuntimeClass {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.this_class.hash(state)
	}
}

impl PartialEq<Self> for RuntimeClass {
	fn eq(&self, other: &Self) -> bool {
		self.this_class.eq(&other.this_class)
	}
}

impl Eq for RuntimeClass {}

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

	pub fn find_field(&self, name: &str, desc: &FieldType) -> Result<&FieldData, VmError> {
		trace!("Finding field");
		if let Some(field) = self.fields.iter().find(|e| {
			trace!("Field Name Needed: {name}, Checked:{}", e.name);
			trace!("Field type Needed: {desc:?}, Checked:{:?}", e.desc);
			let name_match = e.name.eq(name);
			let type_match = *desc == e.desc;
			name_match && type_match
		}) {
			trace!("Found field: {name}");
			return Ok(field);
		};

		// recurse super class
		if let Some(super_class) = &self.super_class {
			return super_class.find_field(name, desc);
		}
		// No field found, and we must be Object, as we don't have a superclass
		Err(VmError::LoaderError("Failed to find field".to_string()))
	}

	pub fn is_assignable_into(&self, into: Arc<RuntimeClass>) -> bool {
		if self.eq(&*into) || self.super_classes.contains(&into) || self.super_interfaces.contains(&into) {
			return true;
		}
		// Array covariance: both must be arrays, then check component types
		if let (Some(self_comp), Some(into_comp)) = (&self.component_type, &into.component_type) {
			// Primitive components must match exactly
			if self_comp.is_primitive_class() {
				return self_comp.eq(&*into_comp);
			}
			// Reference components: recursive covariance
			return self_comp.is_assignable_into(into_comp.clone());
		}

		false
	}

	fn is_primitive_class(&self) -> bool {
		matches!(self.this_class.as_str(), "byte"|"char"|"double"|"float"|"int"|"long"|"short"|"boolean")
	}
}
