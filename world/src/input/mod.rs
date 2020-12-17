mod peripherals_update;
mod peripherals_state;

pub use peripherals_update::*;
pub use peripherals_state::*;

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct RawGamepadState {
	pub stick_left: Vec2f, // the length should be less or equal to 1.0
	pub stick_right: Vec2f, // the length should be less or equal to 1.0
	pub dpad: Vec2f, // x, y coordinates can be -1.0, 0.0 or 1.0
	pub trigger_left: f32, // between 0.0 and 1.0
	pub trigger_right: f32, // between 0.0 and 1.0
	pub bumper_left: bool,
	pub bumper_right: bool,
	pub button_north: bool,
	pub button_west: bool,
	pub button_east: bool,
	pub button_south: bool,
}

impl RawGamepadState {
	pub fn new() -> RawGamepadState {
		RawGamepadState {
			stick_left: Vec2f::new(0.0, 0.0),
			stick_right: Vec2f::new(0.0, 0.0),
			dpad: Vec2f::new(0.0, 0.0),
			trigger_left: 0.0,
			trigger_right: 0.0,
			bumper_left: false,
			bumper_right: false,
			button_north: false,
			button_west: false,
			button_east: false,
			button_south: false,
		}
	}
}
