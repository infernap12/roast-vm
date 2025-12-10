use crate::attributes::ArrayType;
use crate::class::RuntimeClass;
use crate::class_file::ClassFlags;
use crate::objects;
use crate::objects::array::{Array, ArrayReference, ArrayValue, PrimitiveArrayReference};
use crate::objects::object::{Object, ObjectReference, Reference, ReferenceKind};
use crate::rng::generate_identity_hash;
use crate::value::{Primitive, Value};
use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
use log::warn;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct ObjectManager {
	pub objects: HashMap<u32, ReferenceKind>,
	strings: HashMap<String, u32>,
}

impl ObjectManager {
	pub fn new_object(&mut self, class: Arc<RuntimeClass>) -> ObjectReference {
		let id = generate_identity_hash();
		assert!(
			!self.objects.contains_key(&id),
			"Generated ID already exists!"
		);
		let object = Arc::new(Mutex::new(Object {
			id,
			class: class.clone(),
			fields: Default::default(),
		}));
		self.objects.insert(id, ReferenceKind::from(object.clone()));
		object
	}

	pub fn new_primitive_array(&mut self, class: Arc<RuntimeClass>, array_type: ArrayType, count: i32) -> ArrayReference {
		let id = generate_identity_hash();
		assert!(
			!self.objects.contains_key(&id),
			"Generated ID already exists!"
		);

		let array_ref = match array_type {
			ArrayType::T_INT => ArrayReference::Int(Arc::new(Mutex::new(Array::new(id, class,count)))),
			ArrayType::T_BYTE => ArrayReference::Byte(Arc::new(Mutex::new(Array::new(id, class,count)))),
			ArrayType::T_SHORT => {
				ArrayReference::Short(Arc::new(Mutex::new(Array::new(id, class,count))))
			}
			ArrayType::T_LONG => ArrayReference::Long(Arc::new(Mutex::new(Array::new(id, class,count)))),
			ArrayType::T_FLOAT => {
				ArrayReference::Float(Arc::new(Mutex::new(Array::new(id, class,count))))
			}
			ArrayType::T_DOUBLE => {
				ArrayReference::Double(Arc::new(Mutex::new(Array::new(id, class,count))))
			}
			ArrayType::T_CHAR => ArrayReference::Char(Arc::new(Mutex::new(Array::new(id, class,count)))),
			ArrayType::T_BOOLEAN => {
				ArrayReference::Boolean(Arc::new(Mutex::new(Array::new(id, class,count))))
			}
		};

		self.objects
			.insert(id, ReferenceKind::ArrayReference(array_ref.clone()));
		array_ref
	}

	pub fn new_object_array(&mut self, class: Arc<RuntimeClass>, count: i32) -> ArrayReference {
		let id = generate_identity_hash();
		assert!(
			!self.objects.contains_key(&id),
			"Generated ID already exists!"
		);
		let array_ref = ArrayReference::Object(Arc::new(Mutex::new(Array::new(id, class,count))));

		self.objects
			.insert(id, ReferenceKind::ArrayReference(array_ref.clone()));
		array_ref
	}
	pub fn new_byte_array(&mut self, class: Arc<RuntimeClass>, vector: Vec<i8>) -> ArrayReference {
		warn!("Manual sidechannel byte array creation");
		let id = generate_identity_hash();
		assert!(
			!self.objects.contains_key(&id),
			"Generated ID already exists!"
		);
		let array_ref = ArrayReference::Byte(Arc::new(Mutex::new(Array::from((id, class,vector)))));

		self.objects
			.insert(id, ReferenceKind::ArrayReference(array_ref.clone()));
		array_ref
	}

	pub fn get(&self, id: u32) -> ReferenceKind {

		self.objects
			.get(&id)
			.unwrap_or_else(|| {
				let objs = self
					.objects
					.iter()
					.map(|(x, y)| format!("{x} : {y}"))
					.collect::<Vec<_>>();
				panic!("Object must be present id: {id}\n{objs:#?}") })
			.clone()
	}

	pub fn get_interned_string(&self, utf: &str) -> Option<ObjectReference> {
		self.strings
			.get(utf)
			.map(|e| self.get(*e))
			.and_then(ReferenceKind::into_object_reference)
	}

	pub fn new_string(&mut self, byte_class: Arc<RuntimeClass>, string_class: Arc<RuntimeClass>, utf8: &str) -> ObjectReference {
		warn!("Manual sidechannel string creation: \n\"{}\"", utf8);
		let key = utf8.to_owned();
		let jstr = self.new_object(string_class);
		let byte_vec = utf8
			.encode_utf16()
			.flat_map(|e| e.to_le_bytes())
			.map(|e| e as i8)
			.collect::<Vec<_>>();
		let barray = self.new_byte_array(byte_class, byte_vec);

		jstr.lock().unwrap().fields.insert(
			"value".to_string(),
			Value::from(Some(ReferenceKind::ArrayReference(barray))),
		);
		jstr.lock()
			.unwrap()
			.fields
			.insert("coder".to_string(), Value::from(1i8));
		jstr.lock()
			.unwrap()
			.fields
			.insert("hash".to_string(), Value::from(0i32));
		jstr.lock()
			.unwrap()
			.fields
			.insert("hashIsZero".to_string(), Value::from(false));

		let id = jstr.lock().unwrap().id;
		debug_assert!(!self.strings.contains_key(&key), "String already interned");
		self.strings.insert(key, id);
		jstr
	}

	pub fn new_class(
		&mut self,
		class_class: Arc<RuntimeClass>,
		name: Reference,
		module: Reference,
		modifiers: ClassFlags,
		primitive: bool,
	) -> ObjectReference {
		warn!("Manual sidechannel class creation");
		let module = None;
		let modifiers = 17u16;

		let class_redefined_count = 0i32;
		let clazz = self.new_object(class_class);
		if let Ok(clakz) = clazz.lock() {
			clakz.set_field("name", Value::from(name));
			clakz.set_field("module", Value::from(module));
			clakz.set_field("modifiers", Value::from(modifiers));
			clakz.set_field("primitive", Value::from(primitive));
			clakz.set_field("classRedefinedCount", Value::from(class_redefined_count));
		}

		clazz
	}
}
