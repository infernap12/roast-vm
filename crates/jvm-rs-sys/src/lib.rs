#![allow(non_snake_case)]

use jni::objects::{JClass, JString};
use jni::strings::JNIString;
use jni::sys::{jclass, jlong};
use jni::{JNIEnv, NativeMethod};
use std::ffi::c_void;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

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
	let input: String = env
		.get_string(&input)
		.expect("Couldn't get java string!")
		.into();
	std::io::stdout()
		.write_all(input.as_bytes())
		.expect("Failed to emit to stdout");
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
