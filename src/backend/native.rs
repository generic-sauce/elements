use crate::prelude::*;

pub struct NativeBackend;

impl Backend for NativeBackend {
	type InputBackend = NativeInputBackend;
}

pub struct NativeInputBackend {
	pub gilrs: gilrs::Gilrs,
	pub input_receiver: Receiver<PeripheralsUpdate>,
}

pub struct NativeEventIterator<'a> {
	pub gilrs: &'a gilrs::Gilrs,
	pub peripherals_receiver: &'a Receiver<PeripheralsUpdate>,
}

impl<'a> Iterator for NativeEventIterator<'a> {
	type Item = PeripheralsUpdate;

	fn next(&mut self) -> Option<Self::Item> {
		unimplemented!();
	}
}

impl InputBackendTrait for NativeInputBackend {
	type EventIterator<'a> = NativeEventIterator<'a>;

	fn events(&mut self) -> NativeEventIterator<'_> {
		NativeEventIterator {
			gilrs: &self.gilrs,
			peripherals_receiver: &self.input_receiver,
		}
	}

	fn gamepad(&mut self, gamepad_id: u32) -> RawGamepadState {
		if let Some(gamepad) = get_gamepad(gamepad_id, &self.gilrs) {
			let gamepad = self.gilrs.gamepad(gamepad);
			let dpad = Vec2f::new(
				if gamepad.is_pressed(gilrs::Button::DPadRight) { 1.0 } else if gamepad.is_pressed(gilrs::Button::DPadLeft) { -1.0 } else { 0.0 },
				if gamepad.is_pressed(gilrs::Button::DPadUp) { 1.0 } else if gamepad.is_pressed(gilrs::Button::DPadDown) { -1.0 } else { 0.0 },
			);

			RawGamepadState {
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
			RawGamepadState::new()
		}
	}
}

fn get_gamepad(index: u32, gilrs: &gilrs::Gilrs) -> Option<GamepadId> {
	gilrs.gamepads()
		.map(|(gamepad_id, _)| gamepad_id)
		.find(|gamepad_id| Into::<usize>::into(*gamepad_id) == index as usize)
}
