use crate::prelude::*;

pub trait InputBackend {
	fn events(&mut self) -> Vec<PeripheralsUpdate>;
	fn gamepad(&mut self, gamepad_id: u32) -> RawGamepadState;
	fn tick(&mut self);
}
