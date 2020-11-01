#[cfg(feature = "native-client")] mod native;
#[cfg(feature = "native-client")] pub use native::*;

use crate::prelude::*;


pub trait Backend {
	type InputBackend: InputBackend;
	type GraphicsBackend: GraphicsBackend;
}

pub trait InputBackend {
	type EventIterator<'a>: Iterator<Item=PeripheralsUpdate>;

	fn events(&mut self) -> Self::EventIterator<'_>;
	fn gamepad(&mut self, gamepad_id: u32) -> RawGamepadState;
	fn tick(&mut self, peripherals_state: &mut PeripheralsState);
}

pub trait GraphicsBackend {
	fn draw(&mut self, draw: Draw);
}

