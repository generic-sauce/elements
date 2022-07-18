use crate::prelude::*;

pub struct WebInputBackend;

impl InputBackend for WebInputBackend {
	fn events(&mut self) -> Vec<PeripheralsUpdate> { peripherals_events() }

	fn gamepad(&mut self, gamepad_id: u32) -> RawGamepadState {
		gamepad_state(gamepad_id as usize)
	}

	fn tick(&mut self) {}
}
