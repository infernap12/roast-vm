mod primatives;
use std::fs;
use std::fs::File;
use std::io::Read;
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};
use spacetimedb_sdk::__codegen::__lib::SpacetimeType;
use spacetimedb_sdk::__codegen::__sats::bsatn;
use deku_derive

fn main() {
    let mut class_file = File::open("./data/main.class").unwrap();
    let mut bytes = Vec::new();
    class_file.read_to_end(&mut bytes).unwrap();
    let cf = bsatn::from_slice::<ClassFile>(&*bytes).unwrap();
    println!("{:?}", cf);
}



#[deku(magic="")]
pub struct ClassFile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<CpInfo>,  // Note: count is pool.len() + 1
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

// Placeholder types - you'd need to define these based on the JVM spec
#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub enum CpInfo {
    NULL, // i think this is needed?
    Utf8(&'static str),
    NULLTWO, // needed again i think?
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(u16),
    String(u16),
    FieldRef,
    MethodRef(u16, u16),
    InterfaceMethodRef,
    NameAndType,
    NULLTHIRTEEN,
    NULLFOURTEEN,
    MethodHandle,
    MethodType,
    NULLSEVENTEEN,
    InvokeDynamic,
    Module,
    Package
}

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct FieldInfo {
    // Field structure
}

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct MethodInfo {
    // Method structure
}

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct AttributeInfo {
    // Attribute structure
}

pub struct ConstantUtf8Info {
    length: u16,
    bytes: [u8],
    vec: Vec<str>
}