use crate::prelude::*;

const DEADZONE_MIN: f32 = 0.35;
const CURSOR_DEADZONE: f32 = 0.1;

#[derive(Serialize, Deserialize, Clone)]
pub struct InputState {
	pub direction: Vec2f,
	pub cursor: Vec2f,
	pub special1: bool,
	pub special2: bool,
	pub attack1: bool,
	pub attack2: bool,
	pub just_up: bool,
	pub just_down: bool,
	pub just_attack2: bool,
}

impl InputState {
	pub fn new() -> InputState {
		InputState {
			direction: Vec2f::new(0.0, 0.0),
			cursor: Vec2f::new(0.0, 0.0),
			special1: false,
			special2: false,
			attack1: false,
			attack2: false,
			just_up: false,
			just_down: false,
			just_attack2: false,
		}
	}

	pub fn horizontal_dir(&self) -> f32 { self.direction.x }
	pub fn vertical_dir(&self) -> f32 { self.direction.y }

	pub fn up(&self) -> bool { self.vertical_dir() >= DEADZONE_MIN }
	#[allow(unused)]
	pub fn down(&self) -> bool { self.vertical_dir() <= -DEADZONE_MIN }
	pub fn right(&self) -> bool { self.horizontal_dir() >= DEADZONE_MIN }
	pub fn left(&self) -> bool { self.horizontal_dir() <= -DEADZONE_MIN }
	pub fn restart(&self) -> bool {
		self.attack1 || self.special1 || self.left() || self.right() || self.up()
	}

	// returns a value from 0...1000
	pub fn diff(&self, other: &InputState) -> u32 {
		if self.up() != other.up()
			|| self.right() != other.right()
			|| self.left() != other.left()
			|| self.attack1 != other.attack1
			|| self.attack2 != other.attack2
			|| self.special1 != other.special1 {
			return 1000;
		}

		let cursor_diff = (self.cursor - other.cursor).length();
		(1000.0 * cursor_diff).min(1000.0) as u32
	}

	pub fn update(&mut self, current_input: &RawInputState) {
		let last_frame_up = self.up();
		let last_frame_down = self.down();
		let last_frame_attack2 = self.attack2;

		self.direction = Vec2f::new(
			(current_input.dpad.x + apply_deadzone(current_input.stick_left.x)).min(1.0).max(-1.0),
			(current_input.dpad.y + apply_deadzone(current_input.stick_left.y)).min(1.0).max(-1.0)
		);

		self.cursor = Vec2f::new(
			apply_deadzone_min(current_input.stick_right.x, CURSOR_DEADZONE),
			apply_deadzone_min(current_input.stick_right.y, CURSOR_DEADZONE)
		).length_clamped(1.0);

		self.attack1 = current_input.trigger_right > 0.5;
		self.attack2 = current_input.bumper_right;
		self.special1 = current_input.trigger_left > 0.5;
		self.special2 = current_input.bumper_left;

		self.just_up = !last_frame_up && self.up();
		self.just_down = !last_frame_down && self.down();
		self.just_attack2 = !last_frame_attack2 && self.attack2;
	}

	pub fn update_keyboard(&mut self, keyboard_state: &KeyboardState) {
		if keyboard_state.key_pressed(&Key::A) {
			self.direction.x -= 1.0;
		}
		if keyboard_state.key_pressed(&Key::D) {
			self.direction.x += 1.0;
		}

		if keyboard_state.key_pressed(&Key::W) {
			if keyboard_state.key_just_pressed(&Key::W) {
				self.just_up = true;
			}
			self.direction.y += 1.0;
		}
		if keyboard_state.key_pressed(&Key::S) {
			if keyboard_state.key_just_pressed(&Key::S) {
				self.just_down = true;
			}
			self.direction.y -= 1.0;
		}
		self.direction = self.direction.clamped(-1.0, 1.0);

		if keyboard_state.key_pressed(&Key::Q) {
			self.attack1 = true;
		}
		if keyboard_state.key_pressed(&Key::E) {
			self.attack2 = true;
			if keyboard_state.key_just_pressed(&Key::E) {
				self.just_attack2 = true;
			}
		}

		if keyboard_state.key_pressed(&Key::F) {
			self.special1 = true;
		}
		if keyboard_state.key_pressed(&Key::R) {
			self.special2 = true;
		}
	}
}

fn apply_deadzone_min(value: f32, deadzone_min: f32) -> f32 {
	let sign = value.signum();
	let deadzone_max = 1.0 - deadzone_min;
	if value.abs() < deadzone_min {
		0.0
	} else if value.abs() > deadzone_max {
		sign
	} else {
		(value.abs() - deadzone_min) / (deadzone_max-deadzone_min) * sign
	}
}

fn apply_deadzone(value: f32) -> f32 {
	apply_deadzone_min(value, DEADZONE_MIN)
}

