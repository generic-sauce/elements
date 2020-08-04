use crate::prelude::*;

fn get_keyboard_direction(index: u32) -> Vector2f {
	let mut direction = Vector2f::new(0.0, 0.0);
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

fn get_joystick_direction(index: u32) -> Vector2f {
	if !joystick::is_connected(index) {
		return Vector2f::new(0.0, 0.0);
	}
	Vector2f::new(
		joystick::axis_position(index, joystick::Axis::X) * 0.01,
		joystick::axis_position(index, joystick::Axis::Y) * -0.01,
	)
}

pub trait Input {
	fn get_direction(&self) -> Vector2f;
	fn is_connected(&self) -> bool {
		true
	}
}

pub struct KeyboardInput {
	pub index: u32,
}

#[allow(dead_code)]
impl KeyboardInput {
	pub fn new(index: u32) -> KeyboardInput {
		KeyboardInput { index }
	}
}

impl Input for KeyboardInput {
	fn get_direction(&self) -> Vector2f {
		get_keyboard_direction(self.index)
	}
}

pub struct GamePadInput {
	pub index: u32,
}

#[allow(dead_code)]
impl GamePadInput {
	pub fn new(index: u32) -> GamePadInput {
		GamePadInput { index }
	}
}

impl Input for GamePadInput {
	fn get_direction(&self) -> Vector2f {
		get_joystick_direction(self.index)
	}

	fn is_connected(&self) -> bool {
		joystick::is_connected(self.index)
	}
}

pub struct AdaptiveInput {
	pub index: u32,
}

impl AdaptiveInput {
	pub fn new(index: u32) -> AdaptiveInput {
		AdaptiveInput { index }
	}
}

impl Input for AdaptiveInput {
	fn get_direction(&self) -> Vector2f {
		let mut direction = get_keyboard_direction(self.index);
		direction += get_joystick_direction(self.index);
		if direction.x < -1.0 {
			direction.x = -1.0;
		}
		if direction.x > 1.0 {
			direction.x = 1.0;
		}
		if direction.y < -1.0 {
			direction.y = -1.0;
		}
		if direction.y > 1.0 {
			direction.y = 1.0;
		}
		direction
	}
}
