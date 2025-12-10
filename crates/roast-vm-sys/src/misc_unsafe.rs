use jni::sys::{jclass, jint, jobject, jstring, JNIEnv};
use log::warn;
use crate::{get_thread, resolve_object};

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_jdk_internal_misc_Unsafe_registerNatives(
	env: *mut JNIEnv,
	obj: jclass,
) {
	//no op
	()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_jdk_internal_misc_Unsafe_arrayBaseOffset0(
	env: *mut JNIEnv,
	obj: jclass,
) -> jint {
	warn!("arrayBaseOffset0 currently just returning 0");
	0
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_jdk_internal_misc_Unsafe_arrayIndexScale0(
	env: *mut JNIEnv,
	obj: jclass,
) -> jint {
	warn!("arrayIndexScale0 currently just returning 0");
	0
}