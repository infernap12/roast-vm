pub use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
pub trait Primitive {}

impl Primitive for jbyte {}
impl Primitive for jshort {}
impl Primitive for jint {}
impl Primitive for jlong {}
impl Primitive for jchar {}

impl Primitive for jfloat {}
impl Primitive for jdouble {}

impl Primitive for jboolean {}
