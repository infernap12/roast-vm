#![feature(c_variadic)]
use crate::class::RuntimeClass;
use crate::objects::array::ArrayReference;
use crate::objects::object_manager::ObjectManager;
use crate::prim::jboolean;
use crate::thread::VmThread;
use crate::value::{Primitive, Value};
use jni::objects::JClass;
use jni::sys::{jclass, jint, jobject, JNINativeInterface_};
use jni::sys::{jstring, JNIEnv};
use std::ffi::{c_char, CStr, CString};
use std::mem;
use std::ptr;
use std::sync::{Arc, Mutex, RwLock};

const JNI_VERSION_1_1: jint = 0x00010001;
const JNI_VERSION_1_2: jint = 0x00010002;
const JNI_VERSION_1_4: jint = 0x00010004;
const JNI_VERSION_1_6: jint = 0x00010006;
const JNI_VERSION_1_8: jint = 0x00010008;
const JNI_VERSION_9: jint = 0x00090000;
const JNI_VERSION_10: jint = 0x000a0000;
const JNI_VERSION_19: jint = 0x00130000;
const JNI_VERSION_20: jint = 0x00140000;
const JNI_VERSION_21: jint = 0x00150000;
const JNI_VERSION_24: jint = 0x00180000;

pub fn create_jni_function_table(thread: *const VmThread) -> JNIEnv {
	Box::into_raw(Box::new(JNINativeInterface_ {
		reserved0: thread as *mut _,
		reserved1: ptr::null_mut(),
		reserved2: ptr::null_mut(),
		reserved3: ptr::null_mut(),
		GetVersion: Some(jni_get_version),
		DefineClass: Some(define_class),
		FindClass: Some(find_class),
		FromReflectedMethod: Some(from_reflected_method),
		FromReflectedField: Some(from_reflected_field),
		ToReflectedMethod: Some(to_reflected_method),
		GetSuperclass: Some(get_superclass),
		IsAssignableFrom: Some(is_assignable_from),
		ToReflectedField: Some(to_reflected_field),
		Throw: Some(throw),
		ThrowNew: Some(throw_new),
		ExceptionOccurred: Some(exception_occurred),
		ExceptionDescribe: Some(exception_describe),
		ExceptionClear: Some(exception_clear),
		FatalError: Some(fatal_error),
		PushLocalFrame: Some(push_local_frame),
		PopLocalFrame: Some(pop_local_frame),
		NewGlobalRef: Some(new_global_ref),
		DeleteGlobalRef: Some(delete_global_ref),
		DeleteLocalRef: Some(delete_local_ref),
		IsSameObject: Some(is_same_object),
		NewLocalRef: Some(new_local_ref),
		EnsureLocalCapacity: Some(ensure_local_capacity),
		AllocObject: Some(alloc_object),
		NewObject: None,
		NewObjectV: Some(new_object_v),
		NewObjectA: Some(new_object_a),
		GetObjectClass: Some(get_object_class),
		IsInstanceOf: Some(is_instance_of),
		GetMethodID: Some(get_method_id),
		CallObjectMethod: None,
		CallObjectMethodV: Some(call_object_method_v),
		CallObjectMethodA: Some(call_object_method_a),
		CallBooleanMethod: None,
		CallBooleanMethodV: Some(call_boolean_method_v),
		CallBooleanMethodA: Some(call_boolean_method_a),
		CallByteMethod: None,
		CallByteMethodV: Some(call_byte_method_v),
		CallByteMethodA: Some(call_byte_method_a),
		CallCharMethod: None,
		CallCharMethodV: Some(call_char_method_v),
		CallCharMethodA: Some(call_char_method_a),
		CallShortMethod: None,
		CallShortMethodV: Some(call_short_method_v),
		CallShortMethodA: Some(call_short_method_a),
		CallIntMethod: None,
		CallIntMethodV: Some(call_int_method_v),
		CallIntMethodA: Some(call_int_method_a),
		CallLongMethod: None,
		CallLongMethodV: Some(call_long_method_v),
		CallLongMethodA: Some(call_long_method_a),
		CallFloatMethod: None,
		CallFloatMethodV: Some(call_float_method_v),
		CallFloatMethodA: Some(call_float_method_a),
		CallDoubleMethod: None,
		CallDoubleMethodV: Some(call_double_method_v),
		CallDoubleMethodA: Some(call_double_method_a),
		CallVoidMethod: None,
		CallVoidMethodV: Some(call_void_method_v),
		CallVoidMethodA: Some(call_void_method_a),
		CallNonvirtualObjectMethod: None,
		CallNonvirtualObjectMethodV: Some(call_nonvirtual_object_method_v),
		CallNonvirtualObjectMethodA: Some(call_nonvirtual_object_method_a),
		CallNonvirtualBooleanMethod: None,
		CallNonvirtualBooleanMethodV: Some(call_nonvirtual_boolean_method_v),
		CallNonvirtualBooleanMethodA: Some(call_nonvirtual_boolean_method_a),
		CallNonvirtualByteMethod: None,
		CallNonvirtualByteMethodV: Some(call_nonvirtual_byte_method_v),
		CallNonvirtualByteMethodA: Some(call_nonvirtual_byte_method_a),
		CallNonvirtualCharMethod: None,
		CallNonvirtualCharMethodV: Some(call_nonvirtual_char_method_v),
		CallNonvirtualCharMethodA: Some(call_nonvirtual_char_method_a),
		CallNonvirtualShortMethod: None,
		CallNonvirtualShortMethodV: Some(call_nonvirtual_short_method_v),
		CallNonvirtualShortMethodA: Some(call_nonvirtual_short_method_a),
		CallNonvirtualIntMethod: None,
		CallNonvirtualIntMethodV: Some(call_nonvirtual_int_method_v),
		CallNonvirtualIntMethodA: Some(call_nonvirtual_int_method_a),
		CallNonvirtualLongMethod: None,
		CallNonvirtualLongMethodV: Some(call_nonvirtual_long_method_v),
		CallNonvirtualLongMethodA: Some(call_nonvirtual_long_method_a),
		CallNonvirtualFloatMethod: None,
		CallNonvirtualFloatMethodV: Some(call_nonvirtual_float_method_v),
		CallNonvirtualFloatMethodA: Some(call_nonvirtual_float_method_a),
		CallNonvirtualDoubleMethod: None,
		CallNonvirtualDoubleMethodV: Some(call_nonvirtual_double_method_v),
		CallNonvirtualDoubleMethodA: Some(call_nonvirtual_double_method_a),
		CallNonvirtualVoidMethod: None,
		CallNonvirtualVoidMethodV: Some(call_nonvirtual_void_method_v),
		CallNonvirtualVoidMethodA: Some(call_nonvirtual_void_method_a),
		GetFieldID: Some(get_field_id),
		GetObjectField: Some(get_object_field),
		GetBooleanField: Some(get_boolean_field),
		GetByteField: Some(get_byte_field),
		GetCharField: Some(get_char_field),
		GetShortField: Some(get_short_field),
		GetIntField: Some(get_int_field),
		GetLongField: Some(get_long_field),
		GetFloatField: Some(get_float_field),
		GetDoubleField: Some(get_double_field),
		SetObjectField: Some(set_object_field),
		SetBooleanField: Some(set_boolean_field),
		SetByteField: Some(set_byte_field),
		SetCharField: Some(set_char_field),
		SetShortField: Some(set_short_field),
		SetIntField: Some(set_int_field),
		SetLongField: Some(set_long_field),
		SetFloatField: Some(set_float_field),
		SetDoubleField: Some(set_double_field),
		GetStaticMethodID: Some(get_static_method_id),
		CallStaticObjectMethod: None,
		CallStaticObjectMethodV: Some(call_static_object_method_v),
		CallStaticObjectMethodA: Some(call_static_object_method_a),
		CallStaticBooleanMethod: None,
		CallStaticBooleanMethodV: Some(call_static_boolean_method_v),
		CallStaticBooleanMethodA: Some(call_static_boolean_method_a),
		CallStaticByteMethod: None,
		CallStaticByteMethodV: Some(call_static_byte_method_v),
		CallStaticByteMethodA: Some(call_static_byte_method_a),
		CallStaticCharMethod: None,
		CallStaticCharMethodV: Some(call_static_char_method_v),
		CallStaticCharMethodA: Some(call_static_char_method_a),
		CallStaticShortMethod: None,
		CallStaticShortMethodV: Some(call_static_short_method_v),
		CallStaticShortMethodA: Some(call_static_short_method_a),
		CallStaticIntMethod: None,
		CallStaticIntMethodV: Some(call_static_int_method_v),
		CallStaticIntMethodA: Some(call_static_int_method_a),
		CallStaticLongMethod: None,
		CallStaticLongMethodV: Some(call_static_long_method_v),
		CallStaticLongMethodA: Some(call_static_long_method_a),
		CallStaticFloatMethod: None,
		CallStaticFloatMethodV: Some(call_static_float_method_v),
		CallStaticFloatMethodA: Some(call_static_float_method_a),
		CallStaticDoubleMethod: None,
		CallStaticDoubleMethodV: Some(call_static_double_method_v),
		CallStaticDoubleMethodA: Some(call_static_double_method_a),
		CallStaticVoidMethod: None,
		CallStaticVoidMethodV: Some(call_static_void_method_v),
		CallStaticVoidMethodA: Some(call_static_void_method_a),
		GetStaticFieldID: Some(get_static_field_id),
		GetStaticObjectField: Some(get_static_object_field),
		GetStaticBooleanField: Some(get_static_boolean_field),
		GetStaticByteField: Some(get_static_byte_field),
		GetStaticCharField: Some(get_static_char_field),
		GetStaticShortField: Some(get_static_short_field),
		GetStaticIntField: Some(get_static_int_field),
		GetStaticLongField: Some(get_static_long_field),
		GetStaticFloatField: Some(get_static_float_field),
		GetStaticDoubleField: Some(get_static_double_field),
		SetStaticObjectField: Some(set_static_object_field),
		SetStaticBooleanField: Some(set_static_boolean_field),
		SetStaticByteField: Some(set_static_byte_field),
		SetStaticCharField: Some(set_static_char_field),
		SetStaticShortField: Some(set_static_short_field),
		SetStaticIntField: Some(set_static_int_field),
		SetStaticLongField: Some(set_static_long_field),
		SetStaticFloatField: Some(set_static_float_field),
		SetStaticDoubleField: Some(set_static_double_field),
		NewString: Some(new_string),
		GetStringLength: Some(get_string_length),
		GetStringChars: Some(get_string_chars),
		ReleaseStringChars: Some(release_string_chars),
		NewStringUTF: Some(new_string_utf),
		GetStringUTFLength: Some(get_string_utf_length),
		GetStringUTFChars: Some(get_string_utfchars),
		ReleaseStringUTFChars: Some(release_string_utf_chars),
		GetArrayLength: Some(get_array_length),
		NewObjectArray: Some(new_object_array),
		GetObjectArrayElement: Some(get_object_array_element),
		SetObjectArrayElement: Some(set_object_array_element),
		NewBooleanArray: Some(new_boolean_array),
		NewByteArray: Some(new_byte_array),
		NewCharArray: Some(new_char_array),
		NewShortArray: Some(new_short_array),
		NewIntArray: Some(new_int_array),
		NewLongArray: Some(new_long_array),
		NewFloatArray: Some(new_float_array),
		NewDoubleArray: Some(new_double_array),
		GetBooleanArrayElements: Some(get_boolean_array_elements),
		GetByteArrayElements: Some(get_byte_array_elements),
		GetCharArrayElements: Some(get_char_array_elements),
		GetShortArrayElements: Some(get_short_array_elements),
		GetIntArrayElements: Some(get_int_array_elements),
		GetLongArrayElements: Some(get_long_array_elements),
		GetFloatArrayElements: Some(get_float_array_elements),
		GetDoubleArrayElements: Some(get_double_array_elements),
		ReleaseBooleanArrayElements: Some(release_boolean_array_elements),
		ReleaseByteArrayElements: Some(release_byte_array_elements),
		ReleaseCharArrayElements: Some(release_char_array_elements),
		ReleaseShortArrayElements: Some(release_short_array_elements),
		ReleaseIntArrayElements: Some(release_int_array_elements),
		ReleaseLongArrayElements: Some(release_long_array_elements),
		ReleaseFloatArrayElements: Some(release_float_array_elements),
		ReleaseDoubleArrayElements: Some(release_double_array_elements),
		GetBooleanArrayRegion: Some(get_boolean_array_region),
		GetByteArrayRegion: Some(get_byte_array_region),
		GetCharArrayRegion: Some(get_char_array_region),
		GetShortArrayRegion: Some(get_short_array_region),
		GetIntArrayRegion: Some(get_int_array_region),
		GetLongArrayRegion: Some(get_long_array_region),
		GetFloatArrayRegion: Some(get_float_array_region),
		GetDoubleArrayRegion: Some(get_double_array_region),
		SetBooleanArrayRegion: Some(set_boolean_array_region),
		SetByteArrayRegion: Some(set_byte_array_region),
		SetCharArrayRegion: Some(set_char_array_region),
		SetShortArrayRegion: Some(set_short_array_region),
		SetIntArrayRegion: Some(set_int_array_region),
		SetLongArrayRegion: Some(set_long_array_region),
		SetFloatArrayRegion: Some(set_float_array_region),
		SetDoubleArrayRegion: Some(set_double_array_region),
		RegisterNatives: Some(register_natives),
		UnregisterNatives: Some(unregister_natives),
		MonitorEnter: Some(monitor_enter),
		MonitorExit: Some(monitor_exit),
		GetJavaVM: Some(get_java_vm),
		GetStringRegion: Some(get_string_region),
		GetStringUTFRegion: Some(get_string_utf_region),
		GetPrimitiveArrayCritical: Some(get_primitive_array_critical),
		ReleasePrimitiveArrayCritical: Some(release_primitive_array_critical),
		GetStringCritical: Some(get_string_critical),
		ReleaseStringCritical: Some(release_string_critical),
		NewWeakGlobalRef: Some(new_weak_global_ref),
		DeleteWeakGlobalRef: Some(delete_weak_global_ref),
		ExceptionCheck: Some(exception_check),
		NewDirectByteBuffer: Some(new_direct_byte_buffer),
		GetDirectBufferAddress: Some(get_direct_buffer_address),
		GetDirectBufferCapacity: Some(get_direct_buffer_capacity),
		GetObjectRefType: Some(get_object_ref_type),
	}))
}

unsafe fn get_thread(env: *mut JNIEnv) -> *const VmThread {
	(**env).reserved0 as *const VmThread
}

fn resolve_class(thread: &VmThread, clazz: jclass) -> Option<Arc<RuntimeClass>> {
	let loader = thread.loader.lock().unwrap();
	let class_id = clazz as u32;
	loader.class_from_mirror_id(class_id)
}

fn resolve_object(thread: &VmThread, obj: jobject) -> Option<ObjectReference> {
	let gc = thread.gc.read().unwrap();
	let ReferenceKind::ObjectReference(obj_ref) = gc.get(obj as u32) else {
		return None;
	};
	Some(obj_ref.clone())
}

unsafe extern "system" fn jni_get_version(env: *mut JNIEnv) -> jint {
	JNI_VERSION_24
}

unsafe extern "system" fn get_string_utfchars(
	env: *mut JNIEnv,
	str: jstring,
	is_copy: *mut jboolean,
) -> *const c_char {
	trace!("get_string_utfchars");
	// Set is_copy flag - we always return a copy
	// if !is_copy.is_null() {
	// 	*is_copy = 1;
	// }
	let thread = &*get_thread(env);
	let rust_string = {
		let gc = thread.gc.read().unwrap();

		let obj_id = str as u32;

		let obj_ref = match gc.get(obj_id) {
			ReferenceKind::ObjectReference(obj) => obj,
			_ => return ptr::null(),
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
			return ptr::null();
		};

		let array = byte_array.lock().unwrap();

		let bytes: Vec<u8> = array.backing.iter().map(|&b| b as u8).collect();

		let mut utf16_chars = Vec::new();
		for chunk in bytes.chunks_exact(2) {
			let code_unit = u16::from_le_bytes([chunk[0], chunk[1]]);
			utf16_chars.push(code_unit);
		}

		String::from_utf16_lossy(&utf16_chars)
		// All locks dropped here at end of block
	};

	// Convert to C string
	let c_string = match CString::new(rust_string) {
		Ok(s) => s,
		Err(_) => return ptr::null(), // String contains null byte
	};

	// Return pointer - caller is responsible for calling ReleaseStringUTFChars
	c_string.into_raw()
}

unsafe extern "system" fn get_object_class(env: *mut JNIEnv, obj: jobject) -> jclass {
	trace!("get_object_class");
	let thread = &*get_thread(env);
	let gc = thread.gc.read().unwrap();

	let obj_id = obj as u32;

	let ReferenceKind::ObjectReference(obj_ref) = gc.get(obj_id) else {
		return ptr::null_mut();
	};

	let obj_lock = obj_ref.lock().unwrap();
	let class: &Arc<RuntimeClass> = &obj_lock.class;

	*class.mirror.wait() as jclass
}

unsafe extern "system" fn register_natives(
	env: *mut JNIEnv,
	clazz: jclass,
	methods: *const JNINativeMethod,
	n_methods: jint,
) -> jint {
	trace!("register_natives");
	let thread = &*get_thread(env);
	let gc = thread.gc.read().unwrap();
	let class_id = clazz as u32;
	let ReferenceKind::ObjectReference(class_ref) = gc.get(class_id) else {
		return JNI_ERR;
	};
	let class_name = class_ref.lock().unwrap().class.this_class.clone();
	let class_name = class_name.replace("/", "_");

	// let boop = JClass::from_raw(clazz);
	for i in 0..n_methods as usize {
		let native_method = &*methods.add(i);

		// Read the C strings
		let name = CStr::from_ptr(native_method.name).to_str().unwrap();
		let signature = CStr::from_ptr(native_method.signature).to_str().unwrap();
		let fn_ptr = native_method.fnPtr;

		let full_name = format!("Java_{class_name}_{name}");

		if thread.vm.native_methods.contains_key(&full_name) {
			warn!("Native method already registered {full_name}")
		}

		if full_name.contains("Java_java_lang_Class_desiredAssertionStatus0") {
			warn!("MAJOR HACK, figure out what is wrong with desiredAssertionStatus0");
			continue;
		}

		thread
			.vm
			.native_methods
			.insert(full_name.to_owned(), fn_ptr);

		println!(
			"name:{name}, signature:{signature}, fn_ptr{}",
			fn_ptr.is_null()
		)
	}
	JNI_OK
}

// ============================================================================
// JNI FUNCTION STUBS - All unimplemented functions below
// ============================================================================

use crate::class_file::{FieldData, FieldRef};
use crate::objects::object::{Object, ObjectReference, ReferenceKind};
use crate::{BaseType, FieldType, MethodDescriptor};
use jni::sys::*;
use log::{error, info, trace, warn};

unsafe extern "system" fn define_class(
	env: *mut JNIEnv,
	name: *const c_char,
	loader: jobject,
	buf: *const jbyte,
	len: jsize,
) -> jclass {
	todo!("define_class")
}

unsafe extern "system" fn find_class(env: *mut JNIEnv, name: *const c_char) -> jclass {
	trace!("find_class");
	let name_str = CStr::from_ptr(name).to_str().unwrap();
	let thread = &*get_thread(env);
	let class = thread.get_class(name_str).unwrap();
	*class.mirror.wait() as jclass
}

unsafe extern "system" fn from_reflected_method(env: *mut JNIEnv, method: jobject) -> jmethodID {
	todo!("from_reflected_method")
}

unsafe extern "system" fn from_reflected_field(env: *mut JNIEnv, field: jobject) -> jfieldID {
	todo!("from_reflected_field")
}

unsafe extern "system" fn to_reflected_method(
	env: *mut JNIEnv,
	cls: jclass,
	method_id: jmethodID,
	is_static: jboolean,
) -> jobject {
	todo!("to_reflected_method")
}

unsafe extern "system" fn get_superclass(env: *mut JNIEnv, sub: jclass) -> jclass {
	warn!("get_superclass");
	let thread = &*get_thread(env);
	let Some(sup) = resolve_class(thread, sub) else {
		return ptr::null_mut();
	};
	*sup.mirror.wait() as jclass
}

unsafe extern "system" fn is_assignable_from(
	env: *mut JNIEnv,
	sub: jclass,
	sup: jclass,
) -> jboolean {
	warn!("is_assignable_from");
	// Return true if
	// The first and second class arguments refer to the same Java class.
	// The first class is a subclass of the second class.
	// The first class has the second class as one of its interfaces.
	let thread = &*get_thread(env);
	let Some(first) = resolve_class(thread, sub) else {
		return JNI_FALSE;
	};
	let Some(second) = resolve_class(thread, sup) else {
		return JNI_FALSE;
	};
	if first.is_assignable_into(second) {
		JNI_TRUE
	} else {
		JNI_FALSE
	}
}

unsafe extern "system" fn to_reflected_field(
	env: *mut JNIEnv,
	cls: jclass,
	field_id: jfieldID,
	is_static: jboolean,
) -> jobject {
	todo!("to_reflected_field")
}

unsafe extern "system" fn throw(env: *mut JNIEnv, obj: jthrowable) -> jint {
	todo!("throw")
}

unsafe extern "system" fn throw_new(env: *mut JNIEnv, clazz: jclass, msg: *const c_char) -> jint {
	todo!("throw_new")
}

unsafe extern "system" fn exception_occurred(env: *mut JNIEnv) -> jthrowable {
	todo!("exception_occurred")
}

unsafe extern "system" fn exception_describe(env: *mut JNIEnv) {
	todo!("exception_describe")
}

unsafe extern "system" fn exception_clear(env: *mut JNIEnv) {
	todo!("exception_clear")
}

unsafe extern "system" fn fatal_error(env: *mut JNIEnv, msg: *const c_char) -> ! {
	todo!("fatal_error")
}

unsafe extern "system" fn push_local_frame(env: *mut JNIEnv, capacity: jint) -> jint {
	todo!("push_local_frame")
}

unsafe extern "system" fn pop_local_frame(env: *mut JNIEnv, result: jobject) -> jobject {
	todo!("pop_local_frame")
}

unsafe extern "system" fn new_global_ref(env: *mut JNIEnv, lobj: jobject) -> jobject {
	warn!("todo new_global_ref");
	lobj
}

unsafe extern "system" fn delete_global_ref(env: *mut JNIEnv, gref: jobject) {
	warn!("delete_global_ref")
}

unsafe extern "system" fn delete_local_ref(env: *mut JNIEnv, obj: jobject) {
	warn!("delete_local_ref")
}

unsafe extern "system" fn is_same_object(
	env: *mut JNIEnv,
	obj1: jobject,
	obj2: jobject,
) -> jboolean {
	warn!("is_same_object");
	if obj1 as u32 == obj2 as u32 {
		JNI_TRUE
	} else {
		JNI_FALSE
	}
}

unsafe extern "system" fn new_local_ref(env: *mut JNIEnv, ref_: jobject) -> jobject {
	warn!("new_local_ref");
	ref_
}

unsafe extern "system" fn ensure_local_capacity(env: *mut JNIEnv, capacity: jint) -> jint {
	warn!("ensure_local_capacity");
	JNI_OK
}

unsafe extern "system" fn alloc_object(env: *mut JNIEnv, clazz: jclass) -> jobject {
	todo!("alloc_object")
}

unsafe extern "system" fn new_object_v(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jobject {
	todo!("new_object_v")
}

unsafe extern "system" fn new_object_a(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jobject {
	todo!("new_object_a")
}

unsafe extern "system" fn is_instance_of(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
) -> jboolean {
	trace!("is_instance_of");
	let thread = &*get_thread(env);
	let second = resolve_class(thread, clazz).unwrap();
	let obj = resolve_object(thread, obj).unwrap();
	let first = obj.lock().unwrap().class.clone();
	if first.is_assignable_into(second) {
		JNI_TRUE
	} else {
		JNI_FALSE
	}
}

unsafe extern "system" fn get_method_id(
	env: *mut JNIEnv,
	clazz: jclass,
	name: *const c_char,
	sig: *const c_char,
) -> jmethodID {
	trace!("get_method_id");
	let thread = &*get_thread(env);
	let gc = thread.gc.read().unwrap();
	let loader = thread.loader.lock().unwrap();

	let name_str = CStr::from_ptr(name).to_str().unwrap();
	let sig_str = CStr::from_ptr(sig).to_str().unwrap();

	let class_id = clazz as u32;
	let ReferenceKind::ObjectReference(class_ref) = gc.get(class_id) else {
		return ptr::null_mut();
	};

	let Some(runtime_class) = loader.class_from_mirror_id(class_id) else {
		return ptr::null_mut();
	};

	let desc = MethodDescriptor::parse(sig_str).unwrap();
	let method = runtime_class.find_method(name_str, &desc).unwrap();

	// let Value::Reference(Some(ReferenceKind::ObjectReference(class_name_string))) = class_ref.lock().unwrap().get_field("name") else {
	// 	return ptr::null_mut();
	// };
	// class_name_string.lock().unwrap().get_field()

	method as *const _ as jmethodID
}

unsafe extern "system" fn call_object_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: va_list,
) -> jobject {
	todo!("call_object_method_v")
}

unsafe extern "system" fn call_object_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: *const jvalue,
) -> jobject {
	todo!("call_object_method_a")
}

unsafe extern "system" fn call_boolean_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: va_list,
) -> jboolean {
	todo!("call_boolean_method_v")
}

unsafe extern "system" fn call_boolean_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: *const jvalue,
) -> jboolean {
	todo!("call_boolean_method_a")
}

unsafe extern "system" fn call_byte_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: va_list,
) -> jbyte {
	todo!("call_byte_method_v")
}

unsafe extern "system" fn call_byte_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: *const jvalue,
) -> jbyte {
	todo!("call_byte_method_a")
}

unsafe extern "system" fn call_char_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: va_list,
) -> jchar {
	todo!("call_char_method_v")
}

unsafe extern "system" fn call_char_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: *const jvalue,
) -> jchar {
	todo!("call_char_method_a")
}

unsafe extern "system" fn call_short_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: va_list,
) -> jshort {
	todo!("call_short_method_v")
}

unsafe extern "system" fn call_short_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: *const jvalue,
) -> jshort {
	todo!("call_short_method_a")
}

unsafe extern "system" fn call_int_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: va_list,
) -> jint {
	todo!("call_int_method_v")
}

unsafe extern "system" fn call_int_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: *const jvalue,
) -> jint {
	todo!("call_int_method_a")
}

unsafe extern "system" fn call_long_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: va_list,
) -> jlong {
	todo!("call_long_method_v")
}

unsafe extern "system" fn call_long_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: *const jvalue,
) -> jlong {
	todo!("call_long_method_a")
}

unsafe extern "system" fn call_float_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: va_list,
) -> jfloat {
	todo!("call_float_method_v")
}

unsafe extern "system" fn call_float_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: *const jvalue,
) -> jfloat {
	todo!("call_float_method_a")
}

unsafe extern "system" fn call_double_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: va_list,
) -> jdouble {
	todo!("call_double_method_v")
}

unsafe extern "system" fn call_double_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: *const jvalue,
) -> jdouble {
	todo!("call_double_method_a")
}

unsafe extern "system" fn call_void_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: va_list,
) {
	todo!("call_void_method_v")
}

unsafe extern "system" fn call_void_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	method_id: jmethodID,
	args: *const jvalue,
) {
	todo!("call_void_method_a")
}

unsafe extern "system" fn call_nonvirtual_object_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jobject {
	todo!("call_nonvirtual_object_method_v")
}

unsafe extern "system" fn call_nonvirtual_object_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jobject {
	todo!("call_nonvirtual_object_method_a")
}

unsafe extern "system" fn call_nonvirtual_boolean_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jboolean {
	todo!("call_nonvirtual_boolean_method_v")
}

unsafe extern "system" fn call_nonvirtual_boolean_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jboolean {
	todo!("call_nonvirtual_boolean_method_a")
}

unsafe extern "system" fn call_nonvirtual_byte_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jbyte {
	todo!("call_nonvirtual_byte_method_v")
}

unsafe extern "system" fn call_nonvirtual_byte_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jbyte {
	todo!("call_nonvirtual_byte_method_a")
}

unsafe extern "system" fn call_nonvirtual_char_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jchar {
	todo!("call_nonvirtual_char_method_v")
}

unsafe extern "system" fn call_nonvirtual_char_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jchar {
	todo!("call_nonvirtual_char_method_a")
}

unsafe extern "system" fn call_nonvirtual_short_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jshort {
	todo!("call_nonvirtual_short_method_v")
}

unsafe extern "system" fn call_nonvirtual_short_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jshort {
	todo!("call_nonvirtual_short_method_a")
}

unsafe extern "system" fn call_nonvirtual_int_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jint {
	todo!("call_nonvirtual_int_method_v")
}

unsafe extern "system" fn call_nonvirtual_int_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jint {
	todo!("call_nonvirtual_int_method_a")
}

unsafe extern "system" fn call_nonvirtual_long_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jlong {
	todo!("call_nonvirtual_long_method_v")
}

unsafe extern "system" fn call_nonvirtual_long_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jlong {
	todo!("call_nonvirtual_long_method_a")
}

unsafe extern "system" fn call_nonvirtual_float_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jfloat {
	todo!("call_nonvirtual_float_method_v")
}

unsafe extern "system" fn call_nonvirtual_float_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jfloat {
	todo!("call_nonvirtual_float_method_a")
}

unsafe extern "system" fn call_nonvirtual_double_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jdouble {
	todo!("call_nonvirtual_double_method_v")
}

unsafe extern "system" fn call_nonvirtual_double_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jdouble {
	todo!("call_nonvirtual_double_method_a")
}

unsafe extern "system" fn call_nonvirtual_void_method_v(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) {
	todo!("call_nonvirtual_void_method_v")
}

unsafe extern "system" fn call_nonvirtual_void_method_a(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) {
	todo!("call_nonvirtual_void_method_a")
}

unsafe extern "system" fn get_field_id(
	env: *mut JNIEnv,
	clazz: jclass,
	name: *const c_char,
	sig: *const c_char,
) -> jfieldID {
	trace!("get_field_id");
	let thread = &*get_thread(env);
	let gc = thread.gc.read().unwrap();
	let loader = thread.loader.lock().unwrap();

	let name_str = CStr::from_ptr(name).to_str().unwrap();
	let sig_str = CStr::from_ptr(sig).to_str().unwrap();

	let class_id = clazz as u32;
	let ReferenceKind::ObjectReference(class_ref) = gc.get(class_id) else {
		return ptr::null_mut();
	};

	let Some(runtime_class) = loader.class_from_mirror_id(class_id) else {
		return ptr::null_mut();
	};

	let field_type = FieldType::parse(sig_str).unwrap();
	let field_ref = runtime_class.find_field(name_str, &field_type).unwrap();

	// let Value::Reference(Some(ReferenceKind::ObjectReference(class_name_string))) = class_ref.lock().unwrap().get_field("name") else {
	// 	return ptr::null_mut();
	// };
	// class_name_string.lock().unwrap().get_field()

	field_ref as *const _ as jfieldID
}

unsafe extern "system" fn get_object_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
) -> jobject {
	todo!("get_object_field")
}

unsafe extern "system" fn get_boolean_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
) -> jboolean {
	trace!("get_boolean_field");
	let thread = &*get_thread(env);
	let object = resolve_object(thread, obj).unwrap();
	let field_data = &*(field_id as *const FieldData);
	let field_ref = FieldRef{
		class: object.lock().unwrap().class.this_class.clone(),
		name: field_data.name.clone(),
		desc: FieldType::Base(BaseType::Boolean),
	};
	let val = object.lock().unwrap().get_field(&field_ref);
	if let Value::Primitive(Primitive::Boolean(bool)) = val {
		if bool {
			JNI_TRUE
		} else {
			JNI_FALSE
		}
	} else {
		JNI_FALSE
	}
}

unsafe extern "system" fn get_byte_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
) -> jbyte {
	todo!("get_byte_field")
}

unsafe extern "system" fn get_char_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
) -> jchar {
	todo!("get_char_field")
}

unsafe extern "system" fn get_short_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
) -> jshort {
	todo!("get_short_field")
}

unsafe extern "system" fn get_int_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
) -> jint {
	todo!("get_int_field")
}

unsafe extern "system" fn get_long_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
) -> jlong {
	todo!("get_long_field")
}

unsafe extern "system" fn get_float_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
) -> jfloat {
	todo!("get_float_field")
}

unsafe extern "system" fn get_double_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
) -> jdouble {
	todo!("get_double_field")
}

unsafe extern "system" fn set_object_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
	val: jobject,
) {
	todo!("set_object_field")
}

unsafe extern "system" fn set_boolean_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
	val: jboolean,
) {
	todo!("set_boolean_field")
}

unsafe extern "system" fn set_byte_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
	val: jbyte,
) {
	todo!("set_byte_field")
}

unsafe extern "system" fn set_char_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
	val: jchar,
) {
	todo!("set_char_field")
}

unsafe extern "system" fn set_short_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
	val: jshort,
) {
	todo!("set_short_field")
}

unsafe extern "system" fn set_int_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
	val: jint,
) {
	todo!("set_int_field")
}

unsafe extern "system" fn set_long_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
	val: jlong,
) {
	todo!("set_long_field")
}

unsafe extern "system" fn set_float_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
	val: jfloat,
) {
	todo!("set_float_field")
}

unsafe extern "system" fn set_double_field(
	env: *mut JNIEnv,
	obj: jobject,
	field_id: jfieldID,
	val: jdouble,
) {
	todo!("set_double_field")
}

unsafe extern "system" fn get_static_method_id(
	env: *mut JNIEnv,
	clazz: jclass,
	name: *const c_char,
	sig: *const c_char,
) -> jmethodID {
	todo!("get_static_method_id")
}

unsafe extern "system" fn call_static_object_method_v(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jobject {
	todo!("call_static_object_method_v")
}

unsafe extern "system" fn call_static_object_method_a(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jobject {
	todo!("call_static_object_method_a")
}

unsafe extern "system" fn call_static_boolean_method_v(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jboolean {
	todo!("call_static_boolean_method_v")
}

unsafe extern "system" fn call_static_boolean_method_a(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jboolean {
	todo!("call_static_boolean_method_a")
}

unsafe extern "system" fn call_static_byte_method_v(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jbyte {
	todo!("call_static_byte_method_v")
}

unsafe extern "system" fn call_static_byte_method_a(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jbyte {
	todo!("call_static_byte_method_a")
}

unsafe extern "system" fn call_static_char_method_v(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jchar {
	todo!("call_static_char_method_v")
}

unsafe extern "system" fn call_static_char_method_a(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jchar {
	todo!("call_static_char_method_a")
}

unsafe extern "system" fn call_static_short_method_v(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jshort {
	todo!("call_static_short_method_v")
}

unsafe extern "system" fn call_static_short_method_a(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jshort {
	todo!("call_static_short_method_a")
}

unsafe extern "system" fn call_static_int_method_v(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jint {
	todo!("call_static_int_method_v")
}

unsafe extern "system" fn call_static_int_method_a(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jint {
	todo!("call_static_int_method_a")
}

unsafe extern "system" fn call_static_long_method_v(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jlong {
	todo!("call_static_long_method_v")
}

unsafe extern "system" fn call_static_long_method_a(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jlong {
	todo!("call_static_long_method_a")
}

unsafe extern "system" fn call_static_float_method_v(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jfloat {
	todo!("call_static_float_method_v")
}

unsafe extern "system" fn call_static_float_method_a(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jfloat {
	todo!("call_static_float_method_a")
}

unsafe extern "system" fn call_static_double_method_v(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) -> jdouble {
	todo!("call_static_double_method_v")
}

unsafe extern "system" fn call_static_double_method_a(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) -> jdouble {
	todo!("call_static_double_method_a")
}

unsafe extern "system" fn call_static_void_method_v(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: va_list,
) {
	todo!("call_static_void_method_v")
}

unsafe extern "system" fn call_static_void_method_a(
	env: *mut JNIEnv,
	clazz: jclass,
	method_id: jmethodID,
	args: *const jvalue,
) {
	todo!("call_static_void_method_a")
}

unsafe extern "system" fn get_static_field_id(
	env: *mut JNIEnv,
	clazz: jclass,
	name: *const c_char,
	sig: *const c_char,
) -> jfieldID {
	todo!("get_static_field_id")
}

unsafe extern "system" fn get_static_object_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
) -> jobject {
	todo!("get_static_object_field")
}

unsafe extern "system" fn get_static_boolean_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
) -> jboolean {
	trace!("get_static_boolean_field");
	let thread = &*get_thread(env);
	let class = resolve_class(thread, clazz).unwrap();
	let field_ref = &*(field_id as *const FieldData);
	let field_again = class.find_field(&field_ref.name, &field_ref.desc).unwrap();
	let val = field_again.value.lock().unwrap().clone();
	if let Some(Value::Primitive(Primitive::Boolean(bool))) = val {
		if bool {
			JNI_TRUE
		} else {
			JNI_FALSE
		}
	} else {
		JNI_FALSE
	}
}

unsafe extern "system" fn get_static_byte_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
) -> jbyte {
	todo!("get_static_byte_field")
}

unsafe extern "system" fn get_static_char_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
) -> jchar {
	todo!("get_static_char_field")
}

unsafe extern "system" fn get_static_short_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
) -> jshort {
	todo!("get_static_short_field")
}

unsafe extern "system" fn get_static_int_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
) -> jint {
	todo!("get_static_int_field")
}

unsafe extern "system" fn get_static_long_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
) -> jlong {
	todo!("get_static_long_field")
}

unsafe extern "system" fn get_static_float_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
) -> jfloat {
	todo!("get_static_float_field")
}

unsafe extern "system" fn get_static_double_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
) -> jdouble {
	todo!("get_static_double_field")
}

unsafe extern "system" fn set_static_object_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
	value: jobject,
) {
	todo!("set_static_object_field")
}

unsafe extern "system" fn set_static_boolean_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
	value: jboolean,
) {
	todo!("set_static_boolean_field")
}

unsafe extern "system" fn set_static_byte_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
	value: jbyte,
) {
	todo!("set_static_byte_field")
}

unsafe extern "system" fn set_static_char_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
	value: jchar,
) {
	todo!("set_static_char_field")
}

unsafe extern "system" fn set_static_short_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
	value: jshort,
) {
	todo!("set_static_short_field")
}

unsafe extern "system" fn set_static_int_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
	value: jint,
) {
	todo!("set_static_int_field")
}

unsafe extern "system" fn set_static_long_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
	value: jlong,
) {
	todo!("set_static_long_field")
}

unsafe extern "system" fn set_static_float_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
	value: jfloat,
) {
	todo!("set_static_float_field")
}

unsafe extern "system" fn set_static_double_field(
	env: *mut JNIEnv,
	clazz: jclass,
	field_id: jfieldID,
	value: jdouble,
) {
	todo!("set_static_double_field")
}

unsafe extern "system" fn new_string(
	env: *mut JNIEnv,
	unicode: *const jchar,
	len: jsize,
) -> jstring {
	trace!("new_string");
	let thread = &*get_thread(env);
	if unicode.is_null() && len > 0 {
		return ptr::null_mut();
	}
	let chars: &[u16] = std::slice::from_raw_parts(unicode, len as usize);
	let str = String::from_utf16(chars).unwrap_or_else(|_| {
		// Handle invalid UTF-16 - maybe throw exception?
		String::new()
	});
	let str_ref = thread.intern_string(&str); // or non-interned path
	let str_id = str_ref.lock().unwrap().id;

	str_id as jstring
}

unsafe extern "system" fn get_string_length(env: *mut JNIEnv, str: jstring) -> jsize {
	todo!("get_string_length")
}

unsafe extern "system" fn get_string_chars(
	env: *mut JNIEnv,
	str: jstring,
	is_copy: *mut jboolean,
) -> *const jchar {
	todo!("get_string_chars")
}

unsafe extern "system" fn release_string_chars(
	env: *mut JNIEnv,
	str: jstring,
	chars: *const jchar,
) {
	todo!("release_string_chars")
}

unsafe extern "system" fn new_string_utf(env: *mut JNIEnv, utf: *const c_char) -> jstring {
	trace!("new_string_utf");
	let thread = &*get_thread(env);
	// let mut gc = thread.gc.write().unwrap();
	let intern = true;

	let str = CStr::from_ptr(utf).to_str().unwrap();
	info!("{}", str);
	let str_ref = if intern {
		thread.intern_string(str)
	} else {
		let Ok(string_class) = thread.get_class("java/lang/String") else {
			return ptr::null_mut();
		};

		let str_ref = thread.gc.write().unwrap().new_string(string_class, str);
		str_ref
	};

	let str_id = str_ref.lock().unwrap().id;

	str_id as jstring
}

unsafe extern "system" fn get_string_utf_length(env: *mut JNIEnv, str: jstring) -> jsize {
	todo!("get_string_utf_length")
}

unsafe extern "system" fn release_string_utf_chars(
	env: *mut JNIEnv,
	str: jstring,
	chars: *const c_char,
) {
	todo!("release_string_utf_chars")
}

unsafe extern "system" fn get_array_length(env: *mut JNIEnv, array: jarray) -> jsize {
	todo!("get_array_length")
}

unsafe extern "system" fn new_object_array(
	env: *mut JNIEnv,
	len: jsize,
	clazz: jclass,
	init: jobject,
) -> jobjectArray {
	let thread = &*get_thread(env);
	let mut gc = thread.gc.write().unwrap();
	let loader = thread.loader.lock().unwrap();

	let class_id = clazz as u32;
	let ReferenceKind::ObjectReference(class_ref) = gc.get(class_id) else {
		return ptr::null_mut();
	};

	let Some(runtime_class) = loader.class_from_mirror_id(class_id) else {
		return ptr::null_mut();
	};

	let ArrayReference::Object(arr_ref) = gc.new_object_array(len) else {
		return ptr::null_mut();
	};
	let arr_id = arr_ref.lock().unwrap().id;

	// let Value::Reference(Some(ReferenceKind::ObjectReference(class_name_string))) = class_ref.lock().unwrap().get_field("name") else {
	// 	return ptr::null_mut();
	// };
	// class_name_string.lock().unwrap().get_field()

	arr_id as jobjectArray
}

unsafe extern "system" fn get_object_array_element(
	env: *mut JNIEnv,
	array: jobjectArray,
	index: jsize,
) -> jobject {
	todo!("get_object_array_element")
}

unsafe extern "system" fn set_object_array_element(
	env: *mut JNIEnv,
	array: jobjectArray,
	index: jsize,
	val: jobject,
) {
	let thread = &*get_thread(env);
	let arr_id = array as u32;
	let ReferenceKind::ArrayReference(ArrayReference::Object(arr_ref)) =
		thread.gc.read().unwrap().get(arr_id)
	else {
		panic!("Oop")
	};
	let obj_id = val as u32;
	let obj_ref = thread.gc.read().unwrap().get(obj_id);
	arr_ref.lock().unwrap().set(index, Some(obj_ref))
}

unsafe extern "system" fn new_boolean_array(env: *mut JNIEnv, len: jsize) -> jbooleanArray {
	todo!("new_boolean_array")
}

unsafe extern "system" fn new_byte_array(env: *mut JNIEnv, len: jsize) -> jbyteArray {
	todo!("new_byte_array")
}

unsafe extern "system" fn new_char_array(env: *mut JNIEnv, len: jsize) -> jcharArray {
	todo!("new_char_array")
}

unsafe extern "system" fn new_short_array(env: *mut JNIEnv, len: jsize) -> jshortArray {
	todo!("new_short_array")
}

unsafe extern "system" fn new_int_array(env: *mut JNIEnv, len: jsize) -> jintArray {
	todo!("new_int_array")
}

unsafe extern "system" fn new_long_array(env: *mut JNIEnv, len: jsize) -> jlongArray {
	todo!("new_long_array")
}

unsafe extern "system" fn new_float_array(env: *mut JNIEnv, len: jsize) -> jfloatArray {
	todo!("new_float_array")
}

unsafe extern "system" fn new_double_array(env: *mut JNIEnv, len: jsize) -> jdoubleArray {
	todo!("new_double_array")
}

unsafe extern "system" fn get_boolean_array_elements(
	env: *mut JNIEnv,
	array: jbooleanArray,
	is_copy: *mut jboolean,
) -> *mut jboolean {
	todo!("get_boolean_array_elements")
}

unsafe extern "system" fn get_byte_array_elements(
	env: *mut JNIEnv,
	array: jbyteArray,
	is_copy: *mut jboolean,
) -> *mut jbyte {
	todo!("get_byte_array_elements")
}

unsafe extern "system" fn get_char_array_elements(
	env: *mut JNIEnv,
	array: jcharArray,
	is_copy: *mut jboolean,
) -> *mut jchar {
	todo!("get_char_array_elements")
}

unsafe extern "system" fn get_short_array_elements(
	env: *mut JNIEnv,
	array: jshortArray,
	is_copy: *mut jboolean,
) -> *mut jshort {
	todo!("get_short_array_elements")
}

unsafe extern "system" fn get_int_array_elements(
	env: *mut JNIEnv,
	array: jintArray,
	is_copy: *mut jboolean,
) -> *mut jint {
	todo!("get_int_array_elements")
}

unsafe extern "system" fn get_long_array_elements(
	env: *mut JNIEnv,
	array: jlongArray,
	is_copy: *mut jboolean,
) -> *mut jlong {
	todo!("get_long_array_elements")
}

unsafe extern "system" fn get_float_array_elements(
	env: *mut JNIEnv,
	array: jfloatArray,
	is_copy: *mut jboolean,
) -> *mut jfloat {
	todo!("get_float_array_elements")
}

unsafe extern "system" fn get_double_array_elements(
	env: *mut JNIEnv,
	array: jdoubleArray,
	is_copy: *mut jboolean,
) -> *mut jdouble {
	todo!("get_double_array_elements")
}

unsafe extern "system" fn release_boolean_array_elements(
	env: *mut JNIEnv,
	array: jbooleanArray,
	elems: *mut jboolean,
	mode: jint,
) {
	todo!("release_boolean_array_elements")
}

unsafe extern "system" fn release_byte_array_elements(
	env: *mut JNIEnv,
	array: jbyteArray,
	elems: *mut jbyte,
	mode: jint,
) {
	todo!("release_byte_array_elements")
}

unsafe extern "system" fn release_char_array_elements(
	env: *mut JNIEnv,
	array: jcharArray,
	elems: *mut jchar,
	mode: jint,
) {
	todo!("release_char_array_elements")
}

unsafe extern "system" fn release_short_array_elements(
	env: *mut JNIEnv,
	array: jshortArray,
	elems: *mut jshort,
	mode: jint,
) {
	todo!("release_short_array_elements")
}

unsafe extern "system" fn release_int_array_elements(
	env: *mut JNIEnv,
	array: jintArray,
	elems: *mut jint,
	mode: jint,
) {
	todo!("release_int_array_elements")
}

unsafe extern "system" fn release_long_array_elements(
	env: *mut JNIEnv,
	array: jlongArray,
	elems: *mut jlong,
	mode: jint,
) {
	todo!("release_long_array_elements")
}

unsafe extern "system" fn release_float_array_elements(
	env: *mut JNIEnv,
	array: jfloatArray,
	elems: *mut jfloat,
	mode: jint,
) {
	todo!("release_float_array_elements")
}

unsafe extern "system" fn release_double_array_elements(
	env: *mut JNIEnv,
	array: jdoubleArray,
	elems: *mut jdouble,
	mode: jint,
) {
	todo!("release_double_array_elements")
}

unsafe extern "system" fn get_boolean_array_region(
	env: *mut JNIEnv,
	array: jbooleanArray,
	start: jsize,
	len: jsize,
	buf: *mut jboolean,
) {
	todo!("get_boolean_array_region")
}

unsafe extern "system" fn get_byte_array_region(
	env: *mut JNIEnv,
	array: jbyteArray,
	start: jsize,
	len: jsize,
	buf: *mut jbyte,
) {
	todo!("get_byte_array_region")
}

unsafe extern "system" fn get_char_array_region(
	env: *mut JNIEnv,
	array: jcharArray,
	start: jsize,
	len: jsize,
	buf: *mut jchar,
) {
	todo!("get_char_array_region")
}

unsafe extern "system" fn get_short_array_region(
	env: *mut JNIEnv,
	array: jshortArray,
	start: jsize,
	len: jsize,
	buf: *mut jshort,
) {
	todo!("get_short_array_region")
}

unsafe extern "system" fn get_int_array_region(
	env: *mut JNIEnv,
	array: jintArray,
	start: jsize,
	len: jsize,
	buf: *mut jint,
) {
	todo!("get_int_array_region")
}

unsafe extern "system" fn get_long_array_region(
	env: *mut JNIEnv,
	array: jlongArray,
	start: jsize,
	len: jsize,
	buf: *mut jlong,
) {
	todo!("get_long_array_region")
}

unsafe extern "system" fn get_float_array_region(
	env: *mut JNIEnv,
	array: jfloatArray,
	start: jsize,
	len: jsize,
	buf: *mut jfloat,
) {
	todo!("get_float_array_region")
}

unsafe extern "system" fn get_double_array_region(
	env: *mut JNIEnv,
	array: jdoubleArray,
	start: jsize,
	len: jsize,
	buf: *mut jdouble,
) {
	todo!("get_double_array_region")
}

unsafe extern "system" fn set_boolean_array_region(
	env: *mut JNIEnv,
	array: jbooleanArray,
	start: jsize,
	len: jsize,
	buf: *const jboolean,
) {
	todo!("set_boolean_array_region")
}

unsafe extern "system" fn set_byte_array_region(
	env: *mut JNIEnv,
	array: jbyteArray,
	start: jsize,
	len: jsize,
	buf: *const jbyte,
) {
	todo!("set_byte_array_region")
}

unsafe extern "system" fn set_char_array_region(
	env: *mut JNIEnv,
	array: jcharArray,
	start: jsize,
	len: jsize,
	buf: *const jchar,
) {
	todo!("set_char_array_region")
}

unsafe extern "system" fn set_short_array_region(
	env: *mut JNIEnv,
	array: jshortArray,
	start: jsize,
	len: jsize,
	buf: *const jshort,
) {
	todo!("set_short_array_region")
}

unsafe extern "system" fn set_int_array_region(
	env: *mut JNIEnv,
	array: jintArray,
	start: jsize,
	len: jsize,
	buf: *const jint,
) {
	todo!("set_int_array_region")
}

unsafe extern "system" fn set_long_array_region(
	env: *mut JNIEnv,
	array: jlongArray,
	start: jsize,
	len: jsize,
	buf: *const jlong,
) {
	todo!("set_long_array_region")
}

unsafe extern "system" fn set_float_array_region(
	env: *mut JNIEnv,
	array: jfloatArray,
	start: jsize,
	len: jsize,
	buf: *const jfloat,
) {
	todo!("set_float_array_region")
}

unsafe extern "system" fn set_double_array_region(
	env: *mut JNIEnv,
	array: jdoubleArray,
	start: jsize,
	len: jsize,
	buf: *const jdouble,
) {
	todo!("set_double_array_region")
}

unsafe extern "system" fn unregister_natives(env: *mut JNIEnv, clazz: jclass) -> jint {
	todo!("unregister_natives")
}

unsafe extern "system" fn monitor_enter(env: *mut JNIEnv, obj: jobject) -> jint {
	todo!("monitor_enter")
}

unsafe extern "system" fn monitor_exit(env: *mut JNIEnv, obj: jobject) -> jint {
	todo!("monitor_exit")
}

unsafe extern "system" fn get_java_vm(env: *mut JNIEnv, vm: *mut *mut JavaVM) -> jint {
	todo!("get_java_vm")
}

unsafe extern "system" fn get_string_region(
	env: *mut JNIEnv,
	str: jstring,
	start: jsize,
	len: jsize,
	buf: *mut jchar,
) {
	todo!("get_string_region")
}

unsafe extern "system" fn get_string_utf_region(
	env: *mut JNIEnv,
	str: jstring,
	start: jsize,
	len: jsize,
	buf: *mut c_char,
) {
	todo!("get_string_utf_region")
}

unsafe extern "system" fn get_primitive_array_critical(
	env: *mut JNIEnv,
	array: jarray,
	is_copy: *mut jboolean,
) -> *mut std::ffi::c_void {
	todo!("get_primitive_array_critical")
}

unsafe extern "system" fn release_primitive_array_critical(
	env: *mut JNIEnv,
	array: jarray,
	carray: *mut std::ffi::c_void,
	mode: jint,
) {
	todo!("release_primitive_array_critical")
}

unsafe extern "system" fn get_string_critical(
	env: *mut JNIEnv,
	string: jstring,
	is_copy: *mut jboolean,
) -> *const jchar {
	todo!("get_string_critical")
}

unsafe extern "system" fn release_string_critical(
	env: *mut JNIEnv,
	string: jstring,
	cstring: *const jchar,
) {
	todo!("release_string_critical")
}

unsafe extern "system" fn new_weak_global_ref(env: *mut JNIEnv, obj: jobject) -> jweak {
	todo!("new_weak_global_ref")
}

unsafe extern "system" fn delete_weak_global_ref(env: *mut JNIEnv, ref_: jweak) {
	todo!("delete_weak_global_ref")
}

unsafe extern "system" fn exception_check(env: *mut JNIEnv) -> jboolean {
	error!("exception_check");
	JNI_FALSE
}

unsafe extern "system" fn new_direct_byte_buffer(
	env: *mut JNIEnv,
	address: *mut std::ffi::c_void,
	capacity: jlong,
) -> jobject {
	todo!("new_direct_byte_buffer")
}

unsafe extern "system" fn get_direct_buffer_address(
	env: *mut JNIEnv,
	buf: jobject,
) -> *mut std::ffi::c_void {
	todo!("get_direct_buffer_address")
}

unsafe extern "system" fn get_direct_buffer_capacity(env: *mut JNIEnv, buf: jobject) -> jlong {
	todo!("get_direct_buffer_capacity")
}

unsafe extern "system" fn get_object_ref_type(env: *mut JNIEnv, obj: jobject) -> jobjectRefType {
	todo!("get_object_ref_type")
}
