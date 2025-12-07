use jni::sys::{jclass, jint, jobject, jstring, JNIEnv};
use crate::{get_thread, resolve_object};

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_java_lang_Object_hashCode(
	env: *mut JNIEnv,
	obj: jobject,
) -> jint {
	let thread = &*get_thread(env);
	obj as u32 as i32
}