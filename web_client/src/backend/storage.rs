use crate::prelude::*;

use web_sys::Storage;

pub struct WebStorageBackend;

fn storage() -> Storage {
	web_sys::window().unwrap().local_storage().unwrap().unwrap()
}

impl StorageBackend for WebStorageBackend {
	fn set(&mut self, key: &str, value: &str) {
		storage().set(key, value).unwrap();
	}

	fn get(&self, key: &str) -> Option<String> {
		storage().get(key).unwrap()
	}
}
