use crate::prelude::*;

pub struct WebStorageBackend;

impl StorageBackend for WebStorageBackend {
	fn set(&mut self, key: &str, value: &str) {
		set_localstorage(key.to_string(), value.to_string());
	}

	fn get(&self, key: &str) -> Option<String> {
		get_localstorage(key.to_string())
	}
}
