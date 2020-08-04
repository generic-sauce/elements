use sfml::system::Vector2f;
use sfml::window::joystick;

pub trait Input {
	fn get_direction(&self) -> Vector2f;
	fn is_connected(&self) -> bool;
}

pub struct KeyboardInput;

impl KeyboardInput {
	pub fn new() -> KeyboardInput {
		KeyboardInput
	}
}

impl Input for KeyboardInput {
	fn get_direction(&self) -> Vector2f {
		let mut direction = Vector2f::new(0.0, 0.0);
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
		direction
	}

	fn is_connected(&self) -> bool {
		true
	}
}

pub struct GamePadInput {
	pub index: u32,
}

impl GamePadInput {
	pub fn new(index: u32) -> GamePadInput {
		GamePadInput { index }
	}
}

impl Input for GamePadInput {
	fn get_direction(&self) -> Vector2f {
		if !joystick::is_connected(self.index) {
			return Vector2f::new(0.0, 0.0);
		}
		Vector2f::new(
			joystick::axis_position(self.index, joystick::Axis::X) * 0.01,
			joystick::axis_position(self.index, joystick::Axis::Y) * 0.01,
		)
	}

	fn is_connected(&self) -> bool {
		joystick::is_connected(self.index)
	}
}