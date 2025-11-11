use std::ops::Deref;
use crate::class_file::{ConstantClassInfo, ConstantFieldrefInfo, ConstantNameAndTypeInfo, ConstantPoolExt, CpInfo, FieldData, FieldInfo, MethodData, MethodInfo};
use crate::{FieldType, MethodDescriptor};

struct ConstantPool<'a>(&'a [CpInfo]);

impl Deref for ConstantPool<'_> {
    type Target = [CpInfo];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl ConstantPool<'_> {
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