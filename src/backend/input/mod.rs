use crate::prelude::*;

#[cfg(feature = "web-client")] mod web;
#[cfg(feature = "web-client")] pub use web::*;

#[cfg(feature = "native-client")] mod native;
#[cfg(feature = "native-client")] pub use native::*;

pub trait InputBackend {
	type EventIterator<'a>: Iterator<Item=PeripheralsUpdate>;

	fn events(&mut self) -> Self::EventIterator<'_>;
	fn gamepad(&mut self, gamepad_id: u32) -> RawGamepadState;
	fn tick(&mut self);
}

