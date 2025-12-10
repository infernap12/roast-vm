use crate::attributes::{
	Attribute, AttributeInfo, CodeAttribute, LineNumberTableAttribute, LocalVariableTableAttribute,
};
use crate::class_file::{
	ConstantClassInfo, ConstantDynamicInfo, ConstantFieldrefInfo, ConstantInterfaceMethodrefInfo,
	ConstantInvokeDynamicInfo, ConstantMethodHandleInfo, ConstantMethodTypeInfo,
	ConstantMethodrefInfo, ConstantModuleInfo, ConstantNameAndTypeInfo, ConstantPackageInfo,
	ConstantPoolEntry, ConstantStringInfo, ConstantUtf8Info, DescParseError, FieldInfo, FieldRef
	, MethodRef,
};
use crate::{pool_get_impl, FieldType, MethodDescriptor};
use deku::DekuContainerRead;
use std::fmt::{Display, Formatter};
use crate::error::VmError;

pub type ConstantPoolSlice = [ConstantPoolEntry];
pub type ConstantPoolOwned = Vec<ConstantPoolEntry>;

impl ConstantPoolExt for ConstantPoolSlice {}

pub trait ConstantPoolExt: ConstantPoolGet {
	// fn get_constant(&self, index: u16) -> Option<&ConstantPoolEntry> {
	// 	let mut current_index = 1u16;
	// 	for entry in self {
	// 		if current_index == index {
	// 			return Some(entry);
	// 		}
	// 		current_index += match entry {
	// 			ConstantPoolEntry::Long(_) | ConstantPoolEntry::Double(_) => 2,
	// 			_ => 1,
	// 		};
	// 	}
	// 	None
	// }
	//
	fn get_string(&self, index: u16) -> Result<String, ConstantPoolError> {
		let cp_entry = self.get_utf8_info(index)?;

		String::from_utf8(cp_entry.bytes.clone()).map_err(|e| e.to_string().into())
	}
	//
	// fn get_field(&self, index: u16) -> Result<&ConstantFieldrefInfo, ()> {
	// 	let cp_entry = self.get_constant(index).ok_or(())?;
	// 	match cp_entry {
	// 		ConstantPoolEntry::FieldRef(data) => Ok(data),
	// 		_ => Err(()),
	// 	}
	// }
	//
	// fn get_class(&self, index: u16) -> Result<&ConstantClassInfo, ()> {
	// 	let cp_entry = self.get_constant(index).ok_or(())?;
	// 	match cp_entry {
	// 		ConstantPoolEntry::Class(data) => Ok(data),
	// 		_ => Err(()),
	// 	}
	// }
	// fn get_name_and_type(&self, index: u16) -> Result<&ConstantNameAndTypeInfo, ()> {
	// 	let cp_entry = self.get_constant(index).ok_or(())?;
	// 	match cp_entry {
	// 		ConstantPoolEntry::NameAndType(data) => Ok(data),
	// 		_ => Err(()),
	// 	}
	// }

	fn resolve_field(&self, index: u16) -> Result<FieldRef, ConstantPoolError> {
		let fr = self.get_field_ref(index)?;
		let class = self.resolve_class_name(fr.class_index)?;
		let name_and_type = self.get_name_and_type_info(fr.name_and_type_index)?;
		let name = self.get_string(name_and_type.name_index)?;
		let desc = self.get_string(name_and_type.descriptor_index)?;
		let desc = FieldType::parse(&desc)?;
		Ok(FieldRef { class, name, desc })
	}

	/// Resolves a class name from a constant pool class info entry
	///
	/// # Arguments
	/// * `index` - Index into constant pool that must point to a CONSTANT_Class_info structure
	///
	/// # Returns
	/// * Binary class name in internal JVM format (e.g. "java/lang/Object")
	///
	/// # Errors
	/// * Returns ConstantPoolError if index is invalid or points to wrong type
	fn resolve_class_name(&self, index: u16) -> Result<String, ConstantPoolError> {
		let class_info = self.get_class_info(index)?;
		self.get_string(class_info.name_index)
	}

	fn resolve_method_ref(&self, index: u16) -> Result<MethodRef, ConstantPoolError> {
		let mr = self.get_method_ref(index)?;
		let class = self.resolve_class_name(mr.class_index)?;
		let name_and_type = self.get_name_and_type_info(mr.name_and_type_index)?;
		let name = self.get_string(name_and_type.name_index)?;
		let desc = self.get_string(name_and_type.descriptor_index)?;
		let desc = MethodDescriptor::parse(&desc)?;
		Ok(MethodRef { class, name, desc })
	}

	fn resolve_interface_method_ref(&self, index: u16) -> Result<MethodRef, ConstantPoolError> {
		let mr = self.get_interface_method_ref(index)?;
		let class = self.resolve_class_name(mr.class_index)?;
		let name_and_type = self.get_name_and_type_info(mr.name_and_type_index)?;
		let name = self.get_string(name_and_type.name_index)?;
		let desc = self.get_string(name_and_type.descriptor_index)?;
		let desc = MethodDescriptor::parse(&desc)?;
		Ok(MethodRef { class, name, desc })
	}

	/*// (name, desc)
	fn resolve_method_info(&self, method: &MethodInfo) -> Result<MethodData, ConstantPoolError> {
		let desc = self.get_string(method.descriptor_index)?;
		let desc = MethodDescriptor::parse(&desc)?;
		let name = self.get_string(method.name_index)?;
		Ok(MethodData {
			class: "".to_string(),
			name,
			desc,
			code: None,
		})
	}*/

	fn resolve_field_info(&self, field: &FieldInfo) -> Result<FieldRef, ConstantPoolError> {
		let desc = self.get_string(field.descriptor_index)?;
		let desc = FieldType::parse(&desc)?;
		let name = self.get_string(field.name_index)?;
		Ok(FieldRef {
			class: "".to_string(),
			name,
			desc,
		})
	}

	fn parse_attribute(&self, a: AttributeInfo) -> Result<Attribute, VmError> {
		let name = self.get_string(a.attribute_name_index)?;
		// trace!("Parsing attribute with name: {}", name);

		match name.as_ref() {
			"Code" => {
				let (_, mut code_attr) = CodeAttribute::from_bytes((a.info.as_slice(), 0))?;
				Ok(Attribute::Code(code_attr))
			}
			"SourceFile" => {
				let source_file_index = u16::from_be_bytes([a.info[0], a.info[1]]);
				Ok(Attribute::SourceFile(source_file_index))
			}
			"LineNumberTable" => {
				let (_, lnt) = LineNumberTableAttribute::from_bytes((&a.info.as_slice(), 0))?;
				Ok(Attribute::LineNumberTable(lnt))
			}
			"StackMapTable" => Ok(Attribute::StackMapTable(a.info.clone())),
			"Exceptions" => Ok(Attribute::Exceptions(a.info.clone())),
			"InnerClasses" => Ok(Attribute::InnerClasses(a.info.clone())),
			"Signature" => {
				let signature_index = u16::from_be_bytes([a.info[0], a.info[1]]);
				Ok(Attribute::Signature(signature_index))
			}
			"LocalVariableTable" => {
				let (_, lvt) = LocalVariableTableAttribute::from_bytes((&a.info.as_slice(), 0))?;
				Ok(Attribute::LocalVariableTable(lvt))
			}
			_ => Ok(Attribute::Unknown(name.to_string(), a.info.clone())),
		}
	}
}

// pub trait ConstantPoolGet {
// 	fn get_i32(&self, index: u16) -> Result<&i32, ConstantPoolError>;
// 	fn get_f32(&self, index: u16) -> Result<&f32, ConstantPoolError>;
// 	fn get_i64(&self, index: u16) -> Result<&i64, ConstantPoolError>;
// 	fn get_f64(&self, index: u16) -> Result<&f64, ConstantPoolError>;
// 	fn get_utf8_info(&self, index: u16) -> Result<&ConstantUtf8Info, ConstantPoolError>;
// 	fn get_class_info(&self, index: u16) -> Result<&ConstantClassInfo, ConstantPoolError>;
// 	fn get_string_info(&self, index: u16) -> Result<&ConstantStringInfo, ConstantPoolError>;
// 	fn get_field_ref(&self, index: u16) -> Result<&ConstantFieldrefInfo, ConstantPoolError>;
// 	fn get_method_ref(&self, index: u16) -> Result<&ConstantMethodrefInfo, ConstantPoolError>;
// 	fn get_interface_method_ref(&self, index: u16) -> Result<&ConstantInterfaceMethodrefInfo, ConstantPoolError>;
// 	fn get_name_and_type_info(&self, index: u16) -> Result<&ConstantNameAndTypeInfo, ConstantPoolError>;
// 	fn get_method_handle_info(&self, index: u16) -> Result<&ConstantMethodHandleInfo, ConstantPoolError>;
// 	fn get_method_type_info(&self, index: u16) -> Result<&ConstantMethodTypeInfo, ConstantPoolError>;
// 	fn get_dynamic_info(&self, index: u16) -> Result<&ConstantDynamicInfo, ConstantPoolError>;
// 	fn get_invoke_dynamic_info(&self, index: u16) -> Result<&ConstantInvokeDynamicInfo, ConstantPoolError>;
// 	fn get_module_info(&self, index: u16) -> Result<&ConstantModuleInfo, ConstantPoolError>;
// 	fn get_package_info(&self, index: u16) -> Result<&ConstantPackageInfo, ConstantPoolError>;
// }

pub trait ConstantPoolGet: AsRef<[ConstantPoolEntry]> {
	fn get_constant(&self, index: u16) -> Result<&ConstantPoolEntry, ConstantPoolError> {
		let mut current_index = 1u16;
		for entry in self.as_ref() {
			if current_index == index {
				return Ok(entry);
			}
			current_index += match entry {
				ConstantPoolEntry::Long(_) | ConstantPoolEntry::Double(_) => 2,
				_ => 1,
			};
		}
		Err("No constant pool entry at that index".to_string().into())
	}
	pool_get_impl!(get_i32 => i32, Integer);
	pool_get_impl!(get_f32 => f32, Float);
	pool_get_impl!(get_i64 => i64, Long);
	pool_get_impl!(get_f64 => f64, Double);
	pool_get_impl!(get_utf8_info => ConstantUtf8Info, Utf8);
	pool_get_impl!(get_class_info => ConstantClassInfo, Class);
	pool_get_impl!(get_string_info => ConstantStringInfo, String);
	pool_get_impl!(get_field_ref => ConstantFieldrefInfo, FieldRef);
	pool_get_impl!(get_method_ref => ConstantMethodrefInfo, MethodRef);
	pool_get_impl!(get_interface_method_ref => ConstantInterfaceMethodrefInfo, InterfaceMethodRef);
	pool_get_impl!(get_name_and_type_info => ConstantNameAndTypeInfo, NameAndType);
	pool_get_impl!(get_method_handle_info => ConstantMethodHandleInfo, MethodHandle);
	pool_get_impl!(get_method_type_info => ConstantMethodTypeInfo, MethodType);
	pool_get_impl!(get_dynamic_info => ConstantDynamicInfo, Dynamic);
	pool_get_impl!(get_invoke_dynamic_info => ConstantInvokeDynamicInfo, InvokeDynamic);
	pool_get_impl!(get_module_info => ConstantModuleInfo, Module);
	pool_get_impl!(get_package_info => ConstantPackageInfo, Package);
}

impl ConstantPoolGet for [ConstantPoolEntry] {}

impl Display for ConstantPoolError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.0.fmt(f)
	}
}
#[derive(Debug)]
pub struct ConstantPoolError(String);
impl From<String> for ConstantPoolError {
	fn from(value: String) -> Self {
		Self(value)
	}
}

impl From<DescParseError> for ConstantPoolError {
	fn from(value: DescParseError) -> Self {
		value.to_string().into()
	}
}
