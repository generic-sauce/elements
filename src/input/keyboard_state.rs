use crate::prelude::*;

pub struct KeyboardState {
	pub pressed_keys: HashSet<Key>,
	pub just_pressed_keys: HashSet<Key>,
}

impl KeyboardState {
	pub fn new() -> KeyboardState {
		KeyboardState {
			pressed_keys: HashSet::new(),
			just_pressed_keys: HashSet::new(),
		}
	}

	pub fn key_pressed(&self, key: &Key) -> bool {
		self.pressed_keys.contains(key)
	}

	pub fn key_just_pressed(&self, key: &Key) -> bool {
		self.just_pressed_keys.contains(key)
	}

	pub fn update(&mut self, key_update: &KeyboardUpdate) {
		match key_update {
			KeyboardUpdate::KeyPress(key) => { self.update_press(key) },
			KeyboardUpdate::KeyRelease(key) => { self.pressed_keys.remove(key); },
			KeyboardUpdate::Text(_) => {},
		};
	}

	fn update_press(&mut self, key: &Key) {
		if !self.key_pressed(key) {
			self.pressed_keys.insert(*key);
			self.just_pressed_keys.insert(*key);
		}
	}

	fn update_release(&mut self, key: &Key) {
		self.pressed_keys.remove(key);
		self.just_pressed_keys.remove(key);
	}

	pub fn reset_just_pressed(&mut self) {
		self.just_pressed_keys.clear();
	}
}