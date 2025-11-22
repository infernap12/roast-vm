use crate::class::RuntimeClass;
use crate::object::Object;
use crate::rng::generate_identity_hash;
use crate::ObjectRef;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct ObjectManager {
	objects: HashMap<u32, ObjectRef>,
}

impl ObjectManager {
	pub fn new(&mut self, class: Arc<RuntimeClass>) -> ObjectRef {
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
		self.objects.insert(id, object.clone());
		object
	}

	pub fn get(&self, id: u32) -> ObjectRef {
		self.objects
			.get(&id)
			.expect("Object must be present")
			.clone()
	}
}
