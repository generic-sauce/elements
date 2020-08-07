use crate::prelude::*;

const CONTROLLER_MAX: i32 = 100;
const DEADZONE_MIN: i32 = 35;
const DEADZONE_MAX: i32 = CONTROLLER_MAX - DEADZONE_MIN;

fn get_keyboard_direction(index: u32) -> Vec2f {
	let mut direction = Vec2f::new(0.0, 0.0);
	if index == 0 {
		if sfml::window::Key::W.is_pressed() {
			direction.y += 1.0;
		}
		if sfml::window::Key::S.is_pressed() {
			direction.y -= 1.0;
		}

		if sfml::window::Key::D.is_pressed() {
			direction.x += 1.0;
		}
		if sfml::window::Key::A.is_pressed() {
			direction.x -= 1.0;
		}
	} else if index == 1 {
		if sfml::window::Key::Up.is_pressed() {
			direction.y += 1.0;
		}
		if sfml::window::Key::Down.is_pressed() {
			direction.y -= 1.0;
		}

		if sfml::window::Key::Right.is_pressed() {
			direction.x += 1.0;
		}
		if sfml::window::Key::Left.is_pressed() {
			direction.x -= 1.0;
		}
	}
	direction
}

fn get_joystick_direction(index: u32) -> Vec2i {
	if !joystick::is_connected(index) {
		return Vec2i::new(0, 0);
	}
	Vec2i::new(
		joystick::axis_position(index, joystick::Axis::X) as i32,
		joystick::axis_position(index, joystick::Axis::Y) as i32,
	)
}

fn apply_deadzone(value: i32) -> i32 {
	let sign = value.signum();
	if value.abs() < DEADZONE_MIN {
		0
	} else if value.abs() > DEADZONE_MAX {
		sign * CONTROLLER_MAX
	} else {
		(value.abs()*CONTROLLER_MAX / (DEADZONE_MAX-DEADZONE_MIN) - ((CONTROLLER_MAX * DEADZONE_MIN) / (DEADZONE_MAX-DEADZONE_MIN))) * sign
	}
}

pub trait Input {
	fn horizontal_dir(&self) -> i32;
	fn vertical_dir(&self) -> i32;

	fn up(&self) -> bool { self.vertical_dir() >= DEADZONE_MIN }
	fn down(&self) -> bool { self.vertical_dir() <= -DEADZONE_MIN }
	fn right(&self) -> bool { self.horizontal_dir() >= DEADZONE_MIN }
	fn left(&self) -> bool { self.horizontal_dir() <= -DEADZONE_MIN }
	fn attack1(&self) -> bool;
	fn attack2(&self) -> bool;
	fn special1(&self) -> bool;
	fn special2(&self) -> bool;

	fn aim(&self) -> Vec2i;

	fn update(&mut self);
}

// TODO: use bitmask instead of booleans
pub struct AdaptiveInput {
	index: u32,

	direction: Vec2i,
	special1: bool,
	special2: bool,
	attack1: bool,
	attack2: bool,
	aim: Vec2i,
}

impl AdaptiveInput {
	pub fn new(index: u32) -> AdaptiveInput {
		AdaptiveInput {
			index,
			direction: Vec2i::new(0, 0),
			special1: false,
			special2: false,
			attack1: false,
			attack2: false,
			aim: Vec2i::new(0, 0),
		}
	}
}

impl Input for AdaptiveInput {
	fn horizontal_dir(&self) -> i32 {
		self.direction.x
	}

	fn vertical_dir(&self) -> i32 {
		self.direction.y
	}

	fn attack1(&self) -> bool {
		self.attack1
	}

	fn attack2(&self) -> bool {
		self.attack2
	}

	fn special1(&self) -> bool {
		self.special1
	}

	fn special2(&self) -> bool {
		self.special2
	}

	fn aim(&self) -> Vec2i {
		self.aim
	}

	fn update(&mut self) {
		let controller_connected = joystick::is_connected(self.index);

		let up_key = if self.index == 0 { sfml::window::Key::W } else { sfml::window::Key::Up };
		let down_key = if self.index == 0 { sfml::window::Key::S } else { sfml::window::Key::Down };
		let right_key = if self.index == 0 { sfml::window::Key::D } else { sfml::window::Key::Right };
		let left_key = if self.index == 0 { sfml::window::Key::A } else { sfml::window::Key::Left };

		self.direction.x = 0;
		self.direction.y = 0;

		self.direction.y += if up_key.is_pressed() {
			CONTROLLER_MAX
		} else if down_key.is_pressed() {
			-CONTROLLER_MAX
		} else if controller_connected {
			apply_deadzone(-joystick::axis_position(self.index, joystick::Axis::Y) as i32)
		} else {
			0
		};

		let mut key_pressed = false;
		if right_key.is_pressed() {
			self.direction.x += CONTROLLER_MAX;
			key_pressed = true;
		}
		if left_key.is_pressed() {
			self.direction.x -= CONTROLLER_MAX;
			key_pressed = true;
		}
		if !key_pressed && controller_connected {
			self.direction.x = apply_deadzone(joystick::axis_position(self.index, joystick::Axis::X) as i32)
		}
	}
}
