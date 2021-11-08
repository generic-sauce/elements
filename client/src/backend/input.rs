use crate::prelude::*;

pub trait InputBackend {
	type EventIterator<'a>: Iterator<Item=PeripheralsUpdate> where Self: 'a;

	fn events(&mut self) -> Self::EventIterator<'_>;
	fn gamepad(&mut self, gamepad_id: u32) -> RawGamepadState;
	fn tick(&mut self);
}
