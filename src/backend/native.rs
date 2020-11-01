use crate::prelude::*;

pub struct NativeBackend;

impl Backend for NativeBackend {
	type InputBackend = NativeInputBackend;
	type GraphicsBackend = NativeGraphicsBackend;
}

pub struct NativeInputBackend {
	pub gilrs: gilrs::Gilrs,
	pub input_receiver: Receiver<PeripheralsUpdate>,
}

impl NativeInputBackend {
	pub fn new(input_receiver: Receiver<PeripheralsUpdate>) -> NativeInputBackend {
		NativeInputBackend {
			input_receiver,
			gilrs: gilrs::Gilrs::new().unwrap(),
		}
	}
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

impl InputBackend for NativeInputBackend {
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

	fn tick(&mut self, peripherals_state: &mut PeripheralsState) {
		while self.gilrs.next_event().is_some() {};
		let receive = || self.input_receiver.try_recv().map_err(|err| match err {
			TryRecvError::Disconnected => panic!("PeripheralsUpdate Sender disconnected!"),
			x => x,
		});
		while let Ok(peripherals_update) = receive() {
			peripherals_state.update(&peripherals_update);
		}
	}
}

fn get_gamepad(index: u32, gilrs: &gilrs::Gilrs) -> Option<GamepadId> {
	gilrs.gamepads()
		.map(|(gamepad_id, _)| gamepad_id)
		.find(|gamepad_id| Into::<usize>::into(*gamepad_id) == index as usize)
}

pub struct NativeGraphicsBackend {
	pub graphics_sender: Sender<Draw>,
}

impl GraphicsBackend for NativeGraphicsBackend {
	fn draw(&mut self, gw: Draw) {
		self.graphics_sender.send(gw).unwrap();
	}
}