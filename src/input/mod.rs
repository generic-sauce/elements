mod adaptive;
pub use adaptive::*;

use crate::prelude::*;

const DEADZONE_MIN: f32 = 0.35;
const CURSOR_DEADZONE: f32 = 0.1;
const MAX_MOVEMENT_VALUE: i32 = 100;
const JOYSTICK_DISTANCE: i32 = 2600;
const DEFAULT_MOUSE_POSITION: WindowVec = WindowVec::new(300.0, 300.0);

// TODO: use bitmask instead of booleans
pub enum InputDevice {
	Adaptive(AdaptiveInput),
}

impl InputDevice {
	pub fn update(&mut self, gilrs: &Gilrs) -> InputState {
		match self {
			InputDevice::Adaptive(x) => x.update(gilrs),
		}
	}
}
