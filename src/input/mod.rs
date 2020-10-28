#[cfg(feature = "native-client")] mod device;
mod peripherals_update;
mod keyboard_state;

#[cfg(feature = "native-client")] pub use device::*;
pub use peripherals_update::*;
pub use keyboard_state::*;

use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct RawInputState {
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
