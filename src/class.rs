use std::sync::Arc;
use crate::attributes::AttributeInfo;
use crate::class_file::{ClassFlags, CpInfo, FieldData, FieldInfo, MethodInfo, MethodData, ClassFile};

pub struct RuntimeClass {
	pub constant_pool: Arc<Vec<CpInfo>>,
	pub access_flags: ClassFlags,
	pub this_class: String,
	pub super_class: Arc<RuntimeClass>,
	pub interfaces: Vec<Arc<RuntimeClass>>,
	pub fields: Vec<FieldData>,
	pub methods: Vec<MethodData>,
}

impl From<ClassFile> for RuntimeClass {
	fn from(value: ClassFile) -> Self {
		let constant_pool = value.constant_pool.clone();
		let access_flags = ClassFlags::from(value.access_flags);









		Self {
			constant_pool,
			access_flags,
			this_class: "".to_string(),
			super_class: Arc::new(RuntimeClass {}),
			interfaces: vec![],
			fields: vec![],
			methods: vec![],
		}
	}
}