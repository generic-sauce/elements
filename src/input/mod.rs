use crate::prelude::*;

pub struct InputDevice;

impl InputDevice {
	pub fn update(&mut self) -> InputState {
		let [w, a, s, d] = get_wasd().into_serde::<[bool; 4]>().unwrap();
		let x = d as i32 - a as i32;
		let y = (w as i32 - s as i32) * 100;
		InputState {
			direction: GameVec::new(x,y),
			cursor: GameVec::new(0, 0),
			just_up: w,
			just_down: s,
			special1: false,
			special2: false,
			attack1: false,
			attack2: false,
			just_attack2: false,
		}
	}
}


