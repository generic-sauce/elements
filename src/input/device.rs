use crate::prelude::*;

pub struct InputDevice {
	gamepad_id: Option<GamepadId>,
}

impl InputDevice {
	pub fn new(index: u32, gilrs: &gilrs::Gilrs) -> InputDevice {
		InputDevice {
			gamepad_id: get_gamepad(index, gilrs),
		}
	}
}

impl InputDevice {
	pub fn get_state(&mut self, gilrs: &gilrs::Gilrs) -> RawInputState {
		if let Some(gamepad) = self.gamepad_id.map(|x| gilrs.gamepad(x)) {
			let dpad = Vec2f::new(
				if gamepad.is_pressed(gilrs::Button::DPadRight) { 1.0 } else if gamepad.is_pressed(gilrs::Button::DPadLeft) { -1.0 } else { 0.0 },
				if gamepad.is_pressed(gilrs::Button::DPadUp) { 1.0 } else if gamepad.is_pressed(gilrs::Button::DPadDown) { -1.0 } else { 0.0 },
			);

			RawInputState {
				stick_left: Vec2f::new(gamepad.value(gilrs::Axis::LeftStickX), gamepad.value(gilrs::Axis::LeftStickY)),
				stick_right: Vec2f::new(gamepad.value(gilrs::Axis::RightStickX), gamepad.value(gilrs::Axis::RightStickY)),
				dpad,
				trigger_left: if gamepad.is_pressed(gilrs::Button::LeftTrigger2) { 1.0 } else { 0.0 },
				trigger_right: if gamepad.is_pressed(gilrs::Button::RightTrigger2) { 1.0 } else { 0.0 },
				bumper_left: gamepad.is_pressed(gilrs::Button::LeftTrigger),
				bumper_right: gamepad.is_pressed(gilrs::Button::RightTrigger),
				button_north: gamepad.is_pressed(gilrs::Button::North),
				button_west: gamepad.is_pressed(gilrs::Button::West),
				button_east: gamepad.is_pressed(gilrs::Button::East),
				button_south: gamepad.is_pressed(gilrs::Button::South),
			}
		} else {
			RawInputState {
				stick_left: Vec2f::new(0.0, 0.0),
				stick_right: Vec2f::new(0.0, 0.0),
				dpad: Vec2f::new(0.0, 0.0),
				trigger_left: 0.0,
				trigger_right: 0.0,
				bumper_left: false,
				bumper_right: false,
				button_north: false,
				button_west: false,
				button_east: false,
				button_south: false,
			}
		}
	}
}

fn get_gamepad(index: u32, gilrs: &gilrs::Gilrs) -> Option<GamepadId> {
	gilrs.gamepads()
		.map(|(gamepad_id, _)| gamepad_id)
		.find(|gamepad_id| Into::<usize>::into(*gamepad_id) == index as usize)
}
