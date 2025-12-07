use jni::sys::{jclass, jint, jobject, jstring, JNIEnv};
use crate::{get_thread, resolve_object};

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_jdk_internal_misc_Unsafe_registerNatives(
	env: *mut JNIEnv,
	obj: jclass,
) {
	//no op
	()
}