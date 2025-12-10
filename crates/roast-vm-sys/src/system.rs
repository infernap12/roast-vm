use jni::sys::{jclass, jint, jobject, JNIEnv};
use log::warn;
use crate::{get_thread, resolve_array, resolve_object};

#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_java_lang_System_arraycopy(
	env: *mut JNIEnv,
	_ignored: jclass,
	src: jobject,
	src_pos: jint,
	dst: jobject,
	dst_pos: jint,
	length: jint,
) {
	let thread = &*get_thread(env);

	// Check for null pointers
	if src.is_null() || dst.is_null() {
		panic!("NPE!");
		// throw_exception(env, "java/lang/NullPointerException", None);
		return;
	}

	// Resolve JNI handles to actual objects
	let src_arr = resolve_array(thread, src).expect("Was tested non null");
	let dst_arr = resolve_array(thread, dst).expect("Was tested non null");

	// Validate both are arrays
	/*let (Some(src_arr), Some(dst_arr)) = (src_obj.as_array(), dst_obj.as_array()) else {
		panic!("not arrays!");
		throw_exception(env, "java/lang/ArrayStoreException", None);
		return;
	};*/

	let src_len = src_arr.len();
	let dst_len = dst_arr.len();

	// Bounds checking
	if src_pos < 0 || dst_pos < 0 || length < 0
		|| src_pos.checked_add(length).map_or(true, |end| end > src_len)
		|| dst_pos.checked_add(length).map_or(true, |end| end > dst_len)
	{
		panic!("Array index out of bounds!");
		// throw_exception(env, "java/lang/ArrayIndexOutOfBoundsException", None);
		return;
	}


	dst_arr.copy_from(&src_arr, src_pos, dst_pos, length).expect("Array copy error hell");
	// Type compatibility check + copy
}