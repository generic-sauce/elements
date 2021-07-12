use crate::prelude::*;

pub struct WebStorageBackend;

impl StorageBackend for WebStorageBackend {
	fn set(&mut self, key: &str, value: &str) {
		unimplemented!() // TODO
	}

	fn get(&self, key: &str) -> Option<String> {
		unimplemented!() // TODO
	}
}
