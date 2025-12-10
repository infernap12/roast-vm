#![allow(non_snake_case)]

mod class;
mod object;
mod misc_unsafe;
mod system;

use jni::objects::{JClass, JObject, JString};
use jni::strings::JNIString;
use jni::sys::{jclass, jlong, jobject, jobjectArray};
use jni::{JNIEnv, NativeMethod};
use std::ffi::c_void;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use roast_vm_core::objects::array::ArrayReference;
use roast_vm_core::objects::object::ObjectReference;
use roast_vm_core::objects::ReferenceKind;
use roast_vm_core::VmThread;

#[unsafe(no_mangle)]
pub extern "system" fn current_time_millis<'local>(
	mut env: JNIEnv<'local>,
	jclass: JClass<'local>,
) -> jlong {
	println!("Sneaky hobitses has hijacked the native methods he has");

	// SystemTime::now()
	// 	.duration_since(UNIX_EPOCH)
	// 	.unwrap()
	// 	.as_millis() as jlong

	1337i64
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_example_MockIO_print<'local>(
	mut env: JNIEnv<'local>,
	jclass: JClass<'local>,
	input: JString<'local>,
) {
	unsafe {
		let input: String = env
			.get_string_unchecked(&input)
			.expect("Couldn't get java string!")
			.into();
		std::io::stdout()
			.write_all(input.as_bytes())
			.expect("Failed to emit to stdout");
	}
	// println!("Yeetus bageetus! Im printing from rust!")
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_example_MockIO_registerNatives<'local>(
	mut env: JNIEnv<'local>,
	jclass: JClass<'local>,
) {
	let system_methods = vec![NativeMethod {
		name: JNIString::from("currentTimeMillis"),
		sig: JNIString::from("()J"),
		fn_ptr: current_time_millis as *mut c_void,
	}];
	let system_class = env
		.find_class("java/lang/System")
		.expect("Failed to find system class");

	env.register_native_methods(system_class, &system_methods)
		.expect("failed to register method");

	let library_methods = vec![NativeMethod {
		name: JNIString::from("findEntry0"),
		sig: JNIString::from("(JLjava/lang/String;)J"),
		fn_ptr: findEntry0 as *mut c_void,
	}];

	let library_class = env
		.find_class("jdk/internal/loader/NativeLibrary")
		.expect("Failed to find system class");

	// env.register_native_methods(library_class, &library_methods)
	// 	.expect("failed to register method");
}

#[unsafe(no_mangle)]
pub extern "system" fn findEntry0<'local>(
	mut env: JNIEnv<'local>,
	jclass: JClass<'local>,
	handle: jlong,
	name: JString<'local>,
) -> jlong {
	let name: String = env
		.get_string(&name)
		.expect("Couldn't get java string!")
		.into();
	println!("Name: {}, Handle: {}", name, handle);
	0i64
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_example_Main_getTime<'local>(
	mut env: JNIEnv<'local>,
	jclass: JClass<'local>,
) -> jlong {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis() as jlong
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_org_example_MockIO_println<'local>(
	mut env: JNIEnv<'local>,
	jclass: JClass<'local>,
	input: jlong,
) {
	// let input: String = env
	// 	.get_string(&input)
	// 	.expect("Couldn't get java string!")
	// 	.into();
	println!("{input}")
}

// #[unsafe(no_mangle)]
// pub extern "system" fn Java_java_lang_Class_registerNatives<'local>(
// 	mut env: JNIEnv<'local>,
// 	jclass: JClass<'local>,
// ) {
// 	println!("Java_java_lang_Class_registerNatives is NOP")
// }
//
// #[unsafe(no_mangle)]
// pub extern "system" fn Java_java_lang_Object_getClass<'local>(
// 	mut env: JNIEnv<'local>,
// 	this: JObject<'local>,
// ) -> JClass<'local> {
// 	unsafe {
// 		if this.is_null() {
// 			env.throw_new(
// 				"java/lang/NullPointerException",
// 				"Cannot invoke getClass() on null",
// 			)
// 			.unwrap();
// 			return JClass::from_raw(std::ptr::null_mut());
// 		}
// 	}
// 	match env.get_object_class(this) {
// 		Ok(c) => c,
// 		Err(e) => {
// 			eprintln!("get_object_class failed: {:?}", e);
// 			panic!("debug");
// 		}
// 	}
// }

unsafe fn get_thread(env: *mut jni::sys::JNIEnv) -> *const VmThread {
	(**env).reserved0 as *const VmThread
}
fn resolve_object(thread: &VmThread, obj: jobject) -> Option<ObjectReference> {
	let gc = thread.gc.read().unwrap();
	let ReferenceKind::ObjectReference(obj_ref) = gc.get(obj as u32) else {
		return None;
	};
	Some(obj_ref.clone())
}

fn resolve_array(thread: &VmThread, obj: jobject) -> Option<ArrayReference> {
	let gc = thread.gc.read().unwrap();
	let ReferenceKind::ArrayReference(arr_ref) = gc.get(obj as u32) else {
		return None;
	};
	Some(arr_ref.clone())
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_jdk_internal_util_SystemProps_00024Raw_vmProperties<'local>(
	mut env: JNIEnv<'local>,
	_class: JClass<'local>,
) -> jobjectArray {
	let props = [
		"java.vm.specification.name",
		"Java Virtual Machine Specification",
		"java.vm.specification.vendor",
		"Oracle Corporation",
		"java.vm.specification.version",
		"25",
		"java.vm.name",
		"RoastVM",
		"java.vm.vendor",
		"infernap12",
		"java.vm.version",
		"0.1.0",
		"java.vm.info",
		"interpreted mode",
		"jdk.debug",
		"release",
		"java.home",
		"C:\\Program Files\\Java\\jdk-25", // adjust
		"java.library.path",
		"",
		"sun.boot.library.path",
		"",
		"java.class.path",
		".",
	];

	let string_class = env.find_class("java/lang/String").unwrap();
	// +1 for null terminator
	let arr = env
		.new_object_array((props.len() + 1) as i32, string_class, JObject::null())
		.unwrap();

	for (i, s) in props.iter().enumerate() {
		let jstr = env.new_string(s).unwrap();
		env.set_object_array_element(&arr, i as i32, jstr).unwrap();
	}
	// Last element stays null (terminator)

	arr.into_raw()
}
