use deku::ctx::Endian::Big;
use std::borrow::Cow;
use std::fmt;
use std::ops::Deref;
use std::str::Chars;
use std::sync::Arc;
use itertools::Itertools;
use deku_derive::{DekuRead, DekuWrite};
use deku::{DekuContainerRead, DekuError};
use crate::attributes::{Attribute, AttributeInfo, CodeAttribute, Ops};
use crate::{BaseType, FieldType, MethodDescriptor, Value};

#[derive(Debug, PartialEq, DekuRead)]
#[deku(magic = b"\xCA\xFE\xBA\xBE", endian = "big")]
pub struct ClassFile {
	pub minor_version: u16,
	pub major_version: u16,
	constant_pool_count: u16,
	#[deku(
		until = "CpInfo::weighted_count(*constant_pool_count - 1)",
		map = "|v: Vec<CpInfo>| -> Result<_, DekuError> { Ok(Arc::new(v)) }"
	)]
	pub constant_pool: Arc<Vec<CpInfo>>,
	pub access_flags: u16,
	pub this_class: u16,
	pub super_class: u16,
	interfaces_count: u16,
	#[deku(count = "interfaces_count")]
	pub interfaces: Vec<u16>,
	fields_count: u16,
	#[deku(count = "fields_count")]
	pub fields: Vec<FieldInfo>,
	methods_count: u16,
	#[deku(count = "methods_count")]
	pub methods: Vec<MethodInfo>,
	attributes_count: u16,
	#[deku(count = "attributes_count")]
	pub attributes: Vec<AttributeInfo>,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(id_type = "u8", ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub enum CpInfo {
	#[deku(id = 0x01)]
	Utf8(ConstantUtf8Info),
	#[deku(id = 0x03)]
	Integer(i32),
	#[deku(id = 0x04)]
	Float(f32),
	#[deku(id = 0x05)]
	Long(i64),
	#[deku(id = 0x06)]
	Double(f64),
	#[deku(id = 0x07)]
	Class(ConstantClassInfo),
	#[deku(id = 0x08)]
	String(ConstantStringInfo),
	#[deku(id = 0x09)]
	FieldRef(ConstantFieldrefInfo),
	#[deku(id = 10)]
	MethodRef(ConstantMethodrefInfo),
	#[deku(id = 11)]
	InterfaceMethodRef(ConstantInterfaceMethodrefInfo),
	#[deku(id = 12)]
	NameAndType(ConstantNameAndTypeInfo),
	#[deku(id = 15)]
	MethodHandle(ConstantMethodHandleInfo),
	#[deku(id = 16)]
	MethodType(ConstantMethodTypeInfo),
	#[deku(id = 17)]
	Dynamic(ConstantDynamicInfo),
	#[deku(id = 18)]
	InvokeDynamic(ConstantInvokeDynamicInfo),
	#[deku(id = 19)]
	Module(ConstantModuleInfo),
	#[deku(id = 20)]
	Package(ConstantPackageInfo)
}

impl CpInfo {
	fn weighted_count(target: u16) -> impl FnMut(&Self) -> bool {
		let mut count = 0;
		move |entry: &Self| {
			count += match entry {
				CpInfo::Long(_) | CpInfo::Double(_) => 2,
				_ => 1,
			};
			count >= target as usize
		}
	}
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct FieldInfo {
	pub access_flags: u16,
	pub name_index: u16,
	pub descriptor_index: u16,
	attributes_count: u16,
	#[deku(count = "attributes_count")]
	pub attributes: Vec<AttributeInfo>,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct MethodInfo {
	pub access_flags: u16,
	pub name_index: u16,
	pub descriptor_index: u16,
	attributes_count: u16,
	#[deku(count = "attributes_count")]
	pub attributes: Vec<AttributeInfo>,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct ConstantUtf8Info {
	pub length: u16,
	#[deku(count="length")]
	pub bytes: Vec<u8>
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct ConstantFieldrefInfo {
	pub class_index: u16,
	pub name_and_type_index: u16,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct ConstantInterfaceMethodrefInfo {
	pub class_index: u16,
	pub name_and_type_index: u16,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct ConstantMethodrefInfo {
	pub class_index: u16,
	pub name_and_type_index: u16,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct ConstantClassInfo {
	pub name_index: u16,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct ConstantNameAndTypeInfo {
	pub name_index: u16,
	pub descriptor_index: u16,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct ConstantStringInfo {
	pub string_index: u16,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct ConstantMethodHandleInfo {
	pub reference_kind: u8,
	pub reference_index: u16,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct ConstantMethodTypeInfo {
	pub descriptor_index: u16,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct ConstantDynamicInfo {
	pub bootstrap_method_attr_index: u16,
	pub name_and_type_index: u16,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct ConstantInvokeDynamicInfo {
	pub bootstrap_method_attr_index: u16,
	pub name_and_type_index: u16,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct ConstantModuleInfo {
	pub name_index: u16,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct ConstantPackageInfo {
	pub name_index: u16,
}

// Display implementations for better formatting
impl fmt::Display for ClassFile {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(f, "Class File Information:")?;
		writeln!(f, "  Version: {}.{}", self.major_version, self.minor_version)?;
		writeln!(f, "  Access Flags: 0x{:04X}", self.access_flags)?;
		writeln!(f, "  This Class: #{}", self.this_class)?;
		writeln!(f, "  Super Class: #{}", self.super_class)?;
		writeln!(f, "\nConstant Pool ({} entries):", self.constant_pool.len())?;
		for (i, entry) in self.constant_pool().iter().enumerate() {
			if let Some(entry_value) = entry
			{ writeln!(f, "  #{}: {}", i, entry_value)?; }
		}
		writeln!(f, "\nInterfaces ({}):", self.interfaces.len())?;
		for interface in &self.interfaces {
			writeln!(f, "  #{}", interface)?;
		}
		writeln!(f, "\nFields ({}):", self.fields.len())?;
		for (i, field) in self.fields.iter().enumerate() {
			let string_name = self.get_string(field.name_index).unwrap();
			writeln!(f, "  [{}:{}] {}", i, string_name, field)?;
		}
		writeln!(f, "\nMethods ({}):", self.methods.len())?;
		for (i, method) in self.methods.iter().enumerate() {
			let string_name = self.get_string(method.name_index).unwrap();
			writeln!(f, "  [{}:{}] {}", i, string_name, method)?;
			for attribute in &method.attributes {
				write!(f, "  ")?;
				self.format_attribute(f, attribute).expect("TODO: panic message");
				// writeln!(f, "  {}", attribute.get(self).unwrap())?
			}
		}
		writeln!(f, "\nAttributes ({}):", self.attributes.len())?;
		for (i, attr) in self.attributes.iter().enumerate() {
			writeln!(f, "  [{}] name_index=#{}, length={}::: {:?}", i, attr.attribute_name_index, attr.attribute_length, attr.get(self).unwrap())?;
		}
		Ok(())
	}


}

impl fmt::Display for CpInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			CpInfo::Utf8(info) => {
				let s = String::from_utf8_lossy(&info.bytes);
				write!(f, "Utf8 \"{}\"", s)
			}
			CpInfo::Integer(val) => write!(f, "Integer {}", val),
			CpInfo::Float(val) => write!(f, "Float {}", val),
			CpInfo::Long(val) => write!(f, "Long {}", val),
			CpInfo::Double(val) => write!(f, "Double {}", val),
			CpInfo::Class(info) => write!(f, "Class #{}", info.name_index),
			CpInfo::String(info) => write!(f, "String #{}", info.string_index),
			CpInfo::FieldRef(info) => write!(f, "FieldRef #{}.#{}", info.class_index, info.name_and_type_index),
			CpInfo::MethodRef(info) => write!(f, "MethodRef #{}.#{}", info.class_index, info.name_and_type_index),
			CpInfo::InterfaceMethodRef(info) => write!(f, "InterfaceMethodRef #{}.#{}", info.class_index, info.name_and_type_index),
			CpInfo::NameAndType(info) => write!(f, "NameAndType #{}:#{}", info.name_index, info.descriptor_index),
			CpInfo::MethodHandle(info) => write!(f, "MethodHandle kind={} #{}", info.reference_kind, info.reference_index),
			CpInfo::MethodType(info) => write!(f, "MethodType #{}", info.descriptor_index),
			CpInfo::Dynamic(info) => write!(f, "Dynamic #{}.#{}", info.bootstrap_method_attr_index, info.name_and_type_index),
			CpInfo::InvokeDynamic(info) => write!(f, "InvokeDynamic #{}.#{}", info.bootstrap_method_attr_index, info.name_and_type_index),
			CpInfo::Module(info) => write!(f, "Module #{}", info.name_index),
			CpInfo::Package(info) => write!(f, "Package #{}", info.name_index),
		}
	}
}

impl fmt::Display for FieldInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "flags=0x{:04X}, name=#{}, descriptor=#{}, attrs={}",
			   self.access_flags, self.name_index, self.descriptor_index, self.attributes.len())
	}
}

impl fmt::Display for MethodInfo {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let attrs: Vec<_> = self.attributes.iter().map(|x| { x.attribute_name_index }).collect();
		write!(f, "flags=0x{:04X}, name=#{}, descriptor=#{}, attrs={}:{:?}",
			   self.access_flags, self.name_index, self.descriptor_index, self.attributes.len(), attrs)
	}
}

impl ClassFile {

	/// Parse with interpreted attributes
	pub fn from_bytes_interpreted(input: (&[u8], usize)) -> Result<((&[u8], usize), Self), DekuError> {
		let (rest, mut class_file) = Self::from_bytes(input)?;

		// Interpret all attributes in-place
		for field in &mut class_file.fields {
			for attr in &mut field.attributes {
				attr.interpreted = attr.parse_attribute(&class_file.constant_pool);
			}
		}

		for method in &mut class_file.methods {
			for attr in &mut method.attributes {
				attr.interpreted = attr.parse_attribute(&class_file.constant_pool);
			}
		}

		for attr in &mut class_file.attributes {
			attr.interpreted = attr.parse_attribute(&class_file.constant_pool);
		}

		Ok((rest, class_file))
	}
	pub fn constant_pool(&self) -> Vec<Option<CpInfo>> {
		let mut expanded = vec![None]; // Index 0 is unused in JVM

		for entry in self.constant_pool.as_ref() {
			expanded.push(Some(entry.clone()));
			match entry {
				CpInfo::Long(_) | CpInfo::Double(_) => {
					expanded.push(None); // Phantom entry
				}
				_ => {}
			}
		}

		expanded
	}

	pub fn get_constant(&self, index: u16) -> Option<&CpInfo> {
		// More efficient: calculate actual index
		/*let mut current_index = 1u16;
		for entry in &self.constant_pool {
			if current_index == index {
				return Some(entry);
			}
			current_index += match entry {
				CpInfo::Long(_) | CpInfo::Double(_) => 2,
				_ => 1,
			};
		}
		None*/
		self.constant_pool.get_constant(index)
	}

	pub fn get_string(&self, index: u16) -> Result<String, ()> {
		self.constant_pool.get_string(index)
		// if let Some(CpInfo::Utf8(utf)) = self.get_constant(index) {
		//     return Some(String::from_utf8_lossy(&utf.bytes));
		// }
		// None
	}

	fn format_attribute(&self, f: &mut fmt::Formatter<'_>, attr: &AttributeInfo) -> fmt::Result {
		let attribute = attr.get(self).unwrap_or_else(|| panic!("Failed to parse attribute {}", attr));
		match &attribute {
			Attribute::Code(code_attr) => {
				writeln!(f, "  {}", attribute)?;
				for attribute in &code_attr.attributes {
					write!(f, "    ")?;
					self.format_attribute(f, &attribute)?;
				}
				Ok(())
			}
			// Attribute::SourceFile(source_file_index) => {
			//     std::fmt::Display::fmt(&source_file_index, f)
			// }
			// Attribute::LineNumberTable(line_numbers) => {
			//     std::fmt::Display::fmt(&line_numbers, f)
			// }
			// Attribute::StackMapTable(stack_map) => {
			//     stack_map.fmt(f)
			// }
			// Attribute::Exceptions(exceptions) => {
			//     exceptions.fmt(f)
			// }
			// Attribute::InnerClasses(inner_classes) => {
			//     inner_classes.fmt(f)
			// }
			// Attribute::Signature(signature_index) => {
			//     std::fmt::Display::fmt(&signature_index, f)
			// }
			// Attribute::LocalVariableTable(local_var_table) => {
			//     std::fmt::Display::fmt(&local_var_table, f)
			// }
			Attribute::Unknown(name, data) => {
				write!(f, "Unknown attribute '{}', {} bytes", name, data.len())
			}
			_ => {
				writeln!(f, "  {}", attribute)
			}
		}
	}

	pub fn get_code(&self, method_ref_data: MethodData) -> Result<CodeAttribute, ()> {
		for info in self.methods.iter() {
			let data = self.constant_pool.resolve_method_info(info)?;
			let is_same_method_name = data.name.eq(&method_ref_data.name);
			let is_same_param_desc = data.desc.parameters.eq(&method_ref_data.desc.parameters);
			if is_same_method_name && is_same_param_desc {
				for attr in &info.attributes {
					let parsed = attr.parse_attribute(&self.constant_pool);
					if let Some(Attribute::Code(attr)) = parsed {
						return Ok(attr)
					}
				}
			}
		}
		Err(())
	}

	// pub fn get_static_field_value(&self, field_ref: &FieldData) -> Value {
	//     for info in self.fields.iter() {
	//         let data = self.constant_pool.resolve_field_info(info)?;
	//         let is_same_field_name = data.name.eq(&field_ref.name);
	//         let is_same_field_desc = data.desc.eq(&method_ref_data.desc);
	//         if is_same_field_name && is_same_field_desc {
	//
	//         }
	//     }
	// }
}

pub fn pool_get_constant(constant_pool: &[CpInfo], index: u16) -> Option<&CpInfo> {
	// More efficient: calculate actual index
	let mut current_index = 1u16;
	for entry in constant_pool {
		if current_index == index {
			return Some(entry);
		}
		current_index += match entry {
			CpInfo::Long(_) | CpInfo::Double(_) => 2,
			_ => 1,
		};
	}
	None
}

pub fn pool_get_string(constant_pool: &[CpInfo], index: u16) -> Option<Cow<'_, str>> {
	if let Some(CpInfo::Utf8(utf)) = pool_get_constant(constant_pool, index) {
		return Some(String::from_utf8_lossy(&utf.bytes));
	}
	None
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(endian = "Big")]
pub(crate) struct Bytecode {
	bytes: u32,
	#[deku(bytes_read = "bytes")]
	pub code: Vec<Ops>,
}



pub trait ConstantPoolExt {
	fn get_constant(&self, index: u16) -> Option<&CpInfo>;
	fn get_string(&self, index: u16) -> Result<String, ()>;
	fn get_field(&self, index: u16) -> Result<&ConstantFieldrefInfo, ()>;
	fn get_class(&self, index: u16) -> Result<&ConstantClassInfo, ()>;
	fn get_name_and_type(&self, index: u16) -> Result<&ConstantNameAndTypeInfo, ()>;
	fn resolve_field(&self, index: u16) -> Result<FieldData, ()>;
	fn resolve_method_ref(&self, index: u16) -> Result<MethodData, ()>;
	fn resolve_method_info(&self, method: &MethodInfo) -> Result<MethodData, ()>;
	fn resolve_field_info(&self, field: &FieldInfo) -> Result<FieldData, ()>;
}

impl ConstantPoolExt for [CpInfo] {
	fn get_constant(&self, index: u16) -> Option<&CpInfo> {
		let mut current_index = 1u16;
		for entry in self {
			if current_index == index {
				return Some(entry);
			}
			current_index += match entry {
				CpInfo::Long(_) | CpInfo::Double(_) => 2,
				_ => 1,
			};
		}
		None
	}

	fn get_string(&self, index: u16) -> Result<String, ()> {
		let cp_entry = self.get_constant(index).ok_or(())?;
		match cp_entry {
			CpInfo::Utf8(data) => {
				String::from_utf8(data.bytes.clone()).map_err(|e| ())
			},
			_ => Err(()),
		}
	}

	fn get_field(&self, index: u16) -> Result<&ConstantFieldrefInfo, ()> {
		let cp_entry = self.get_constant(index).ok_or(())?;
		match cp_entry {
			CpInfo::FieldRef(data) => Ok(data),
			_ => Err(()),
		}
	}

	fn get_class(&self, index: u16) -> Result<&ConstantClassInfo, ()> {
		let cp_entry = self.get_constant(index).ok_or(())?;
		match cp_entry {
			CpInfo::Class(data) => Ok(data),
			_ => Err(()),
		}
	}
	fn get_name_and_type(&self, index: u16) -> Result<&ConstantNameAndTypeInfo, ()> {
		let cp_entry = self.get_constant(index).ok_or(())?;
		match cp_entry {
			CpInfo::NameAndType(data) => Ok(data),
			_ => Err(()),
		}
	}

	fn resolve_field(&self, index: u16) -> Result<FieldData, ()> {
		if let Some(CpInfo::FieldRef(fr)) = self.get_constant(index) {
			let class = self.get_class(fr.class_index)?;
			let class = self.get_string(class.name_index)?;
			let name_and_type = self.get_name_and_type(fr.name_and_type_index)?;
			let name = self.get_string(name_and_type.name_index)?;
			let desc = self.get_string(name_and_type.descriptor_index)?;
			let desc = FieldType::parse(&desc)?;
			Ok(FieldData {
				class,
				name,
				desc,
			})
		} else { Err(()) }
	}

	fn resolve_method_ref(&self, index: u16) -> Result<MethodData, ()> {
		if let Some(CpInfo::MethodRef(mr)) = self.get_constant(index) {
			let class = self.get_class(mr.class_index)?;
			let class = self.get_string(class.name_index)?;
			let name_and_type = self.get_name_and_type(mr.name_and_type_index)?;
			let name = self.get_string(name_and_type.name_index)?;
			let desc = self.get_string(name_and_type.descriptor_index)?;
			let desc = MethodDescriptor::parse(&desc)?;
			Ok(MethodData {
				class,
				name,
				desc,
			})
		} else { Err(()) }
	}

	// (name, desc)
	fn resolve_method_info(&self, method: &MethodInfo) -> Result<MethodData, ()> {
		let desc = self.get_string(method.descriptor_index)?;
		let desc = MethodDescriptor::parse(&desc)?;
		let name = self.get_string(method.name_index)?;
		Ok(MethodData {
			class: "".to_string(),
			name,
			desc,
		})
	}

	fn resolve_field_info(&self, field: &FieldInfo) -> Result<FieldData, ()> {
		let desc = self.get_string(field.descriptor_index)?;
		let desc = FieldType::parse(&desc)?;
		let name = self.get_string(field.name_index)?;
		Ok(FieldData {
			class: "".to_string(),
			name,
			desc,
		})
	}
}


#[derive(Debug)]
pub struct MethodData {
	pub class: String,
	pub name: String,
	pub desc: MethodDescriptor,
	pub code: Option<Bytecode>
}



#[derive(Debug)]
pub struct FieldData {
	pub class: String,
	pub name: String,
	pub desc: FieldType
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct ClassFlags {
	// flags
	#[deku(bits = 1)]
	MODULE: bool,
	#[deku(bits = 1)]
	ENUM: bool,
	#[deku(bits = 1)]
	ANNOTATION: bool,
	#[deku(bits = 1)]
	SYNTHETIC: bool,
	#[deku(bits = 1, pad_bits_before = "1")]
	ABSTRACT: bool,
	#[deku(bits = 1)]
	INTERFACE: bool,
	#[deku(bits = 1, pad_bits_before = "3")]
	SUPER: bool,
	#[deku(bits = 1)]
	FINAL: bool,
	#[deku(bits = 1, pad_bits_before = "3")]
	PUBLIC: bool,
}

impl From<u16> for ClassFlags {
	fn from(value: u16) -> Self {
		let (_rem, flags) = Self::from_bytes((&value.to_be_bytes(), 0)).unwrap();
		flags
	}
}



#[allow(non_snake_case)]
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct ModuleFlags {
	#[deku(bits = 1)]
	ACC_MANDATED: bool,
	#[deku(bits = 1, pad_bits_before = "2")]
	ACC_SYNTHETIC: bool,
	#[deku(bits = 1, pad_bits_before = "5")]
	ACC_STATIC_PHASE: bool,
	#[deku(bits = 1, pad_bits_after = "5")]
	ACC_TRANSITIVE: bool,
}

impl From<u16> for ModuleFlags {
	fn from(value: u16) -> Self {
		let (_rem, flags) = Self::from_bytes((&value.to_be_bytes(), 0)).unwrap();
		flags
	}
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct FieldFlags {
	#[deku(bits = 1, pad_bits_before = "1")]
	ACC_ENUM: bool,
	#[deku(bits = 1, pad_bits_before = "1")]
	ACC_SYNTHETIC: bool,
	#[deku(bits = 1, pad_bits_before = "4")]
	ACC_TRANSIENT: bool,
	#[deku(bits = 1)]
	ACC_VOLATILE: bool,
	#[deku(bits = 1, pad_bits_before = "1")]
	ACC_FINAL: bool,
	#[deku(bits = 1)]
	ACC_STATIC: bool,
	#[deku(bits = 1)]
	ACC_PROTECTED: bool,
	#[deku(bits = 1)]
	ACC_PRIVATE: bool,
	#[deku(bits = 1)]
	ACC_PUBLIC: bool,
}

impl From<u16> for FieldFlags {
	fn from(value: u16) -> Self {
		let (_rem, flags) = Self::from_bytes((&value.to_be_bytes(), 0)).unwrap();
		flags
	}
}

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct MethodFlags {
	#[deku(bits = 1, pad_bits_before = "3")]
	ACC_SYNTHETIC: bool,
	#[deku(bits = 1)]
	ACC_STRICT: bool,
	#[deku(bits = 1)]
	ACC_ABSTRACT: bool,
	#[deku(bits = 1, pad_bits_before = "1")]
	ACC_NATIVE: bool,
	#[deku(bits = 1)]
	ACC_VARARGS: bool,
	#[deku(bits = 1)]
	ACC_BRIDGE: bool,
	#[deku(bits = 1)]
	ACC_SYNCHRONIZED: bool,
	#[deku(bits = 1)]
	ACC_FINAL: bool,
	#[deku(bits = 1)]
	ACC_STATIC: bool,
	#[deku(bits = 1)]
	ACC_PROTECTED: bool,
	#[deku(bits = 1)]
	ACC_PRIVATE: bool,
	#[deku(bits = 1)]
	ACC_PUBLIC: bool,
}

impl From<u16> for MethodFlags {
	fn from(value: u16) -> Self {
		let (_rem, flags) = Self::from_bytes((&value.to_be_bytes(), 0)).unwrap();
		flags
	}
}


//yoinked because im monkled
impl MethodDescriptor {
	/// Parses a method descriptor as specified in the JVM specs:
	/// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.3.3
	pub fn parse(descriptor: &str) -> Result<MethodDescriptor, ()> {
		let mut chars = descriptor.chars();
		match chars.next() {
			Some('(') => {
				let parameters = Self::parse_parameters(descriptor, &mut chars)?;
				if Some(')') == chars.next() {
					let return_type = Self::parse_return_type(descriptor, &mut chars)?;
					Ok(MethodDescriptor {
						parameters,
						return_type,
					})
				} else {
					Err(())
				}
			}
			_ => Err(()),
		}
	}

	fn parse_parameters(
		descriptor: &str,
		chars: &mut Chars,
	) -> Result<Vec<FieldType>, ()> {
		let mut parameters = Vec::new();
		loop {
			match chars.clone().next() {
				Some(')') => return Ok(parameters),
				Some(_) => {
					let param = FieldType::parse_from(descriptor, chars)?;
					parameters.push(param);
				}
				None => return Err(()),
			}
		}
	}

	fn parse_return_type(
		descriptor: &str,
		chars: &mut Chars,
	) -> Result<Option<FieldType>, ()> {
		match chars.clone().next() {
			Some('V') => Ok(None),
			Some(_) => {
				let return_type = Some(FieldType::parse_from(descriptor, chars)?);
				if chars.next().is_none() {
					Ok(return_type)
				} else {
					Err(())
				}
			}
			_ => Err(()),
		}
	}

	pub fn num_arguments(&self) -> usize {
		self.parameters.len()
	}
}

impl FieldType {
	pub fn parse(type_descriptor: &str) -> Result<FieldType, ()> {
		let mut chars = type_descriptor.chars();
		let descriptor = Self::parse_from(type_descriptor, &mut chars)?;
		match chars.next() {
			None => Ok(descriptor),
			Some(_) => Err(()),
		}
	}

	pub(crate) fn parse_from(
		type_descriptor: &str,
		chars: &mut Chars,
	) -> Result<FieldType, ()> {
		let first_char = chars
			.next()
			.ok_or(())?;

		Ok(match first_char {
			'B' => FieldType::Base(BaseType::Byte),
			'C' => FieldType::Base(BaseType::Char),
			'D' => FieldType::Base(BaseType::Double),
			'F' => FieldType::Base(BaseType::Float),
			'I' => FieldType::Base(BaseType::Int),
			'J' => FieldType::Base(BaseType::Long),
			'S' => FieldType::Base(BaseType::Short),
			'Z' => FieldType::Base(BaseType::Boolean),
			'L' => {
				let class_name: String = chars.take_while_ref(|c| *c != ';').collect();
				match chars.next() {
					Some(';') => FieldType::ClassType(class_name),
					_ => return Err(()),
				}
			}
			'[' => {
				let component_type = Self::parse_from(type_descriptor, chars)?;
				FieldType::ArrayType(Box::new(component_type))
			}
			_ => return Err(()),
		})
	}
}