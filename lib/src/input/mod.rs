mod adaptive;
pub use adaptive::*;

use crate::prelude::*;

const DEADZONE_MIN: f32 = 0.35;
const CURSOR_DEADZONE: f32 = 0.07;
const MAX_MOVEMENT_VALUE: i32 = 100;
const JOYSTICK_DISTANCE: i32 = 2600;
const DEFAULT_MOUSE_POSITION: Vec2i = Vec2i::new(300, 300);

// TODO: use bitmask instead of booleans
pub enum InputDevice {
	Adaptive(AdaptiveInput),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InputState {
	pub direction: GameVec,
	pub cursor: GameVec,
	pub just_up: bool,
	pub just_down: bool,
	pub special1: bool,
	pub special2: bool,
	pub attack1: bool,
	pub attack2: bool,
}

impl InputDevice {
	pub fn update(&mut self, gilrs: &Gilrs) -> InputState {
		match self {
			InputDevice::Adaptive(x) => x.update(gilrs),
		}
	}
}

impl InputState {
	pub fn new() -> InputState {
		InputState {
			direction: Default::default(),
			cursor: Default::default(),
			just_up: false,
			just_down: false,
			special1: false,
			special2: false,
			attack1: false,
			attack2: false,
		}
	}

	pub fn horizontal_dir(&self) -> i32 { self.direction.x }
	pub fn vertical_dir(&self) -> i32 { self.direction.y }

	pub fn up(&self) -> bool { self.vertical_dir() >= (DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }
	#[allow(unused)]
	pub fn down(&self) -> bool { self.vertical_dir() <= (-DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }
	pub fn right(&self) -> bool { self.horizontal_dir() >= (DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }
	pub fn left(&self) -> bool { self.horizontal_dir() <= (-DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }
}
