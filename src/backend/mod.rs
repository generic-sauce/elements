#[cfg(feature = "native-client")] mod native;

use crate::prelude::*;

pub trait Backend {
	type InputBackend: InputBackendTrait;
}

pub struct PeripheralsUpdateIterator {

}

pub trait InputBackendTrait {
	type EventIterator<'a>: Iterator<Item=PeripheralsUpdate>;

	fn events(&mut self) -> Self::EventIterator<'_>;
	fn gamepad(&mut self, gamepad_id: u32) -> RawGamepadState;
}

