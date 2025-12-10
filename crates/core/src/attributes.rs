use crate::class_file::constant_pool::ConstantPoolExt;
use crate::class_file::{ClassFile, Constant, ConstantPoolEntry};
use deku::DekuContainerRead;
use deku_derive::DekuRead;
use log::trace;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct AttributeInfo {
	pub attribute_name_index: u16,
	pub attribute_length: u32,
	#[deku(count = "attribute_length")]
	pub info: Vec<u8>,
	#[deku(skip)]
	pub interpreted: Option<Attribute>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Attribute {
	// "Critical"
	ConstantValue(Constant),
	Code(CodeAttribute),
	StackMapTable(Vec<u8>),
	BootstrapMethods,
	NestHost,
	NestMembers,
	PermittedSubclasses,

	SourceFile(u16),
	LineNumberTable(LineNumberTableAttribute),
	Exceptions(Vec<u8>),
	InnerClasses(Vec<u8>),
	Signature(u16),
	LocalVariableTable(LocalVariableTableAttribute),
	Unknown(String, Vec<u8>),
}

//noinspection SpellCheckingInspection
#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(id_type = "u8", ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub enum ArrayType {
	#[deku(id = "4")]
	T_BOOLEAN,
	#[deku(id = "5")]
	T_CHAR,
	#[deku(id = "6")]
	T_FLOAT,
	#[deku(id = "7")]
	T_DOUBLE,
	#[deku(id = "8")]
	T_BYTE,
	#[deku(id = "9")]
	T_SHORT,
	#[deku(id = "10")]
	T_INT,
	#[deku(id = "11")]
	T_LONG,
}

impl Display for ArrayType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let str = match self {
			ArrayType::T_BOOLEAN => "[Z",
			ArrayType::T_CHAR => "[C",
			ArrayType::T_FLOAT => "[F",
			ArrayType::T_DOUBLE => "[D",
			ArrayType::T_BYTE => "[B",
			ArrayType::T_SHORT => "[S",
			ArrayType::T_INT => "[I",
			ArrayType::T_LONG => "[J",
		};
		write!(f, "{}", str)
	}
}

// impl TryFrom<u8> for Ops {
//     type Error = ();
//
//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         match value {
//             0x2A => Ok(Ops::Aload0),
//             0x2B => Ok(Ops::InvokeSpecial(0)),
//             0x10 => Ok(Ops::Bipush(0)),
//             0xB5 => Ok(Ops::Putfield(0)),
//             0x14 => Ok(Ops::Ldc2_w(0)),
//             0xB1 => Ok(Ops::Retern),
//             _ => Err(())
//         }
//     }
// }

impl AttributeInfo {
	// pub fn parse_attribute(&self, constant_pool: &[ConstantPoolEntry]) -> Option<Attribute> {
	// 	let name = crate::class_file::pool_get_string(constant_pool, self.attribute_name_index)?;
	// 	trace!("Parsing attribute with name: {}", name);
	//
	//
	// 	match name.as_ref() {
	// 		"Code" => {
	// 			let (_, mut code_attr) = CodeAttribute::from_bytes((&self.info.as_slice(), 0)).ok()?;
	// 			// Recursively interpret nested attributes
	// 			for attr in &mut code_attr.attributes {
	// 				attr.interpreted = attr.parse_attribute(constant_pool);
	// 			}
	// 			Some(Attribute::Code(code_attr))
	// 		}
	// 		"SourceFile" => {
	// 			if self.info.len() >= 2 {
	// 				let source_file_index = u16::from_be_bytes([self.info[0], self.info[1]]);
	// 				Some(Attribute::SourceFile(source_file_index))
	// 			} else {
	// 				None
	// 			}
	// 		}
	// 		"LineNumberTable" => {
	// 			let (_, lnt) = LineNumberTableAttribute::from_bytes((&self.info.as_slice(), 0)).ok()?;
	// 			Some(Attribute::LineNumberTable(lnt))
	// 		}
	// 		"StackMapTable" => {
	// 			Some(Attribute::StackMapTable(self.info.clone()))
	// 		}
	// 		"Exceptions" => {
	// 			Some(Attribute::Exceptions(self.info.clone()))
	// 		}
	// 		"InnerClasses" => {
	// 			Some(Attribute::InnerClasses(self.info.clone()))
	// 		}
	// 		"Signature" => {
	// 			if self.info.len() >= 2 {
	// 				let signature_index = u16::from_be_bytes([self.info[0], self.info[1]]);
	// 				Some(Attribute::Signature(signature_index))
	// 			} else {
	// 				None
	// 			}
	// 		}
	// 		"LocalVariableTable" => {
	// 			let (_, lvt) = LocalVariableTableAttribute::from_bytes((&self.info.as_slice(), 0)).ok()?;
	// 			Some(Attribute::LocalVariableTable(lvt))
	// 		}
	// 		_ => Some(Attribute::Unknown(name.to_string(), self.info.clone())),
	// 	}
	// }

	/// Get the interpreted attribute, parsing if necessary
	pub fn get(&self, class_file: &ClassFile) -> Option<Attribute> {
		class_file.constant_pool.parse_attribute(self.clone()).ok()

		// if let Some(ref attr) = self.interpreted {
		// 	Some(attr.clone())
		// } else {
		// 	self.parse_attribute(class_file.constant_pool.as_ref())
		// }
	}
}

impl LocalVariableTableAttribute {}

impl Display for AttributeInfo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(
			f,
			"AttributeInfo {{ name_index: {}, length: {} }}",
			self.attribute_name_index, self.attribute_length
		)
	}
}

impl Display for Attribute {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Attribute::Code(code) => write!(f, "Code attribute: {}", code),
			Attribute::SourceFile(index) => write!(f, "SourceFile attribute, index: {}", index),
			Attribute::LineNumberTable(data) => write!(f, "LineNumberTable attribute: {}", data),
			Attribute::StackMapTable(data) => {
				write!(f, "StackMapTable attribute, {} bytes", data.len())
			}
			Attribute::Exceptions(data) => write!(f, "Exceptions attribute, {} bytes", data.len()),
			Attribute::InnerClasses(data) => {
				write!(f, "InnerClasses attribute, {} bytes", data.len())
			}
			Attribute::Signature(index) => write!(f, "Signature attribute, index: {}", index),
			Attribute::LocalVariableTable(table) => {
				write!(f, "LocalVariableTable attribute: {}", table)
			}
			Attribute::Unknown(name, data) => {
				write!(f, "Unknown attribute '{}', {} bytes", name, data.len())
			}
			_ => {
				unreachable!()
			}
		}
	}
}

impl Display for CodeAttribute {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"stack={}, locals={}, code={} bytes, exceptions={}, attributes={}",
			self.max_stack,
			self.max_locals,
			self.code_length,
			self.exception_table_length,
			self.attributes_count
		)
	}
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(endian = "big")]
pub struct CodeAttribute {
	pub max_stack: u16,
	pub max_locals: u16,
	pub code_length: u32,
	#[deku(count = "code_length")]
	pub code: Vec<u8>,
	pub exception_table_length: u16,
	#[deku(count = "exception_table_length")]
	pub exception_table: Vec<ExceptionTableEntry>,
	pub attributes_count: u16,
	#[deku(count = "attributes_count")]
	pub attributes: Vec<AttributeInfo>,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct ExceptionTableEntry {
	pub start_pc: u16,
	pub end_pc: u16,
	pub handler_pc: u16,
	pub catch_type: u16,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(endian = "big")]
pub struct LocalVariableTableAttribute {
	pub local_variable_table_length: u16,
	#[deku(count = "local_variable_table_length")]
	pub local_variable_table: Vec<LocalVariableTableEntry>,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct LocalVariableTableEntry {
	pub start_pc: u16,
	pub length: u16,
	pub name_index: u16,
	pub descriptor_index: u16,
	pub index: u16,
}

impl Display for LocalVariableTableAttribute {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"local_variable_table_length={}, entries={}",
			self.local_variable_table_length,
			self.local_variable_table.len()
		)
	}
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(endian = "big")]
pub struct LineNumberTableAttribute {
	pub line_number_table_length: u16,
	#[deku(count = "line_number_table_length")]
	pub line_number_table: Vec<LineNumberTableEntry>,
}

#[derive(Clone, PartialEq, Debug, DekuRead)]
#[deku(ctx = "_endian: deku::ctx::Endian", endian = "big")]
pub struct LineNumberTableEntry {
	pub start_pc: u16,
	pub line_number: u16,
}

impl Display for LineNumberTableAttribute {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"table_length={}, entries={}",
			self.line_number_table_length,
			self.line_number_table.len()
		)
	}
}
