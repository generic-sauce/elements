use crate::prelude::*;

pub struct WebInputBackend;

pub struct WebEventIterator<'a> {
	payload: Vec<PeripheralsUpdate>,
	phantom: PhantomData<&'a ()>,
}

impl<'a> Iterator for WebEventIterator<'a> {
	type Item = PeripheralsUpdate;

	fn next(&mut self) -> Option<Self::Item> {
		if self.payload.is_empty() { return None; }

		Some(self.payload.remove(0))
	}
}

impl InputBackend for WebInputBackend {
	type EventIterator<'a> = WebEventIterator<'a>;

	fn events(&mut self) -> WebEventIterator<'_> {
		WebEventIterator {
			payload: peripherals_events(),
			phantom: PhantomData
		}
	}

	fn gamepad(&mut self, gamepad_id: u32) -> RawGamepadState {
		gamepad_state(gamepad_id as usize)
	}

	fn tick(&mut self) {}
}
