pub trait StorageBackend {
	fn set(&mut self, key: &str, value: &str);
	fn get(&self, key: &str) -> Option<String>;
}
