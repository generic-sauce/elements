use crate::prelude::*;

pub struct InputDevice;

impl InputDevice {
	pub fn update(&mut self) -> InputState {
		InputState {
			direction: GameVec::new(0,0),
			cursor: GameVec::new(0, 0),
			just_up: true,
			just_down: false,
			special1: false,
			special2: false,
			attack1: false,
			attack2: false,
			just_attack2: false,
		}
	}
}


