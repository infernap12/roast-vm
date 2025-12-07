use roast_vm_core::class_file::FieldRef;
use std::ptr;
use jni::sys::jstring;
use jni::sys::JNIEnv;
use jni::sys::jclass;
use roast_vm_core::{BaseType, FieldType};
use roast_vm_core::objects::array::ArrayReference;
use roast_vm_core::objects::ReferenceKind;
use roast_vm_core::value::Value;
use crate::get_thread;

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_java_lang_Class_getPrimitiveClass<'local>(
	env: *mut JNIEnv,
	_class: jclass,
	name: jstring,
) -> jclass {
	let thread = &*get_thread(env);

	let rust_string = {
		let gc = thread.gc.read().unwrap();

		let obj_id = name as u32;

		let obj_ref = match gc.get(obj_id) {
			ReferenceKind::ObjectReference(obj) => obj,
			_ => return 0 as jclass,
		};

		let obj = obj_ref.lock().unwrap();
		let field_ref = FieldRef{
			class: "java/lang/String".to_string(),
			name: "value".to_string(),
			desc: FieldType::ArrayType(Box::new(FieldType::Base(BaseType::Byte))),
		};
		let value_field = obj.get_field(&field_ref);
		let Value::Reference(Some(ReferenceKind::ArrayReference(ArrayReference::Byte(byte_array)))) =
			value_field
		else {
			return 0 as jclass;
		};

		let array = byte_array.lock().unwrap();

		let bytes: Vec<u8> = (&array).iter().map(|&b| b as u8).collect();

		let mut utf16_chars = Vec::new();
		for chunk in bytes.chunks_exact(2) {
			let code_unit = u16::from_le_bytes([chunk[0], chunk[1]]);
			utf16_chars.push(code_unit);
		}

		String::from_utf16_lossy(&utf16_chars)
		// All locks dropped here at end of block
	};

	let klass = thread.get_class(&rust_string).unwrap();
	let class = thread.gc.read().unwrap().get(*klass.mirror.wait());

	class.id() as jclass


}