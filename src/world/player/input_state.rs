use crate::prelude::*;

const DEADZONE_MIN: f32 = 0.35;
const MAX_MOVEMENT_VALUE: i32 = 100;

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
	pub just_attack2: bool,
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
			just_attack2: false,
		}
	}

	pub fn horizontal_dir(&self) -> i32 { self.direction.x }
	pub fn vertical_dir(&self) -> i32 { self.direction.y }

	pub fn up(&self) -> bool { self.vertical_dir() >= (DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }
	#[allow(unused)]
	pub fn down(&self) -> bool { self.vertical_dir() <= (-DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }
	pub fn right(&self) -> bool { self.horizontal_dir() >= (DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }
	pub fn left(&self) -> bool { self.horizontal_dir() <= (-DEADZONE_MIN * MAX_MOVEMENT_VALUE as f32) as i32 }
	pub fn restart(&self) -> bool {
		self.attack1 || self.special1
	}

	// returns a value from 0..=1000
	pub fn diff(&self, other: &InputState) -> u32 {
		if self.up() != other.up()
			|| self.right() != other.right()
			|| self.left() != other.left()
			|| self.attack1 != other.attack1
			|| self.attack2 != other.attack2
			|| self.special1 != other.special1 {
			return 1000;
		}

		let game_length = (self.cursor - other.cursor).length();
		let tile_length = game_length / TILESIZE;
		(1000 * tile_length).min(1000) as u32
	}
}
