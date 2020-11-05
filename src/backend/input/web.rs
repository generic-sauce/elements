use crate::prelude::*;

pub struct WebInputBackend;

pub struct WebEventIterator<'a> {
	phantom: PhantomData<&'a ()>,
}

impl<'a> Iterator for WebEventIterator<'a> {
	type Item = PeripheralsUpdate;

	fn next(&mut self) -> Option<Self::Item> { None }
}

impl InputBackend for WebInputBackend {
	type EventIterator<'a> = WebEventIterator<'a>;

	fn events(&mut self) -> WebEventIterator<'_> {
		WebEventIterator {
			phantom: PhantomData
		}
	}

	fn gamepad(&mut self, gamepad_id: u32) -> RawGamepadState {
		input_state(gamepad_id as usize)
	}

	fn tick(&mut self) {}
}
