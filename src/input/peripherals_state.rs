use crate::prelude::*;

pub struct PeripheralsState {
	pub pressed_keys: HashSet<Key>,
	pub just_pressed_keys: HashSet<Key>,
	pub cursor_move: WindowVec,
}

impl PeripheralsState {
	pub fn new() -> PeripheralsState {
		PeripheralsState {
			pressed_keys: HashSet::new(),
			just_pressed_keys: HashSet::new(),
			cursor_move: WindowVec::new(0.0, 0.0),
		}
	}

	pub fn key_pressed(&self, key: &Key) -> bool {
		self.pressed_keys.contains(key)
	}

	pub fn key_just_pressed(&self, key: &Key) -> bool {
		self.just_pressed_keys.contains(key)
	}

	pub fn update(&mut self, key_update: &PeripheralsUpdate) {
		match key_update {
			PeripheralsUpdate::KeyPress(key) => self.update_press(key),
			PeripheralsUpdate::KeyRelease(key) => self.update_release(key),
			PeripheralsUpdate::MouseMove(cursor_move) => {
				self.cursor_move.x += cursor_move.x;
				self.cursor_move.y += cursor_move.y;
			},
			_ => {},
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

	pub fn reset(&mut self) {
		self.just_pressed_keys.clear();
		self.cursor_move = WindowVec::new(0.0, 0.0);
	}
}