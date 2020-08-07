mod draw;
mod physics;
pub mod sensor;

use crate::prelude::*;

// The left-bottom of the tile (x,y) is located at position (x,y) * TILESIZE.
pub const PLAYER_SIZE: GameVec = TileVec::new(2, 6).to_game();

const X_DRAG: i32 = 30;
const MAX_X_VEL: i32 = 120;
const JUMP_POWER: i32 = 300;
const WALLJUMP_POWER: i32 = JUMP_POWER;
const X_ACCELERATION: i32 = 55;

// also required for fluids!
pub const GRAVITY: i32 = 15;

static GROUND_SENSOR: Sensor = Sensor {
	left_bot_offset: GameVec::new(0, -1),
	size: GameVec::new(PLAYER_SIZE.x, 2),
};

static LEFT_SENSOR: Sensor = Sensor {
	left_bot_offset: GameVec::new(-TILESIZE/2, PLAYER_SIZE.y / 4),
	size: GameVec::new(0, PLAYER_SIZE.y * 3 / 4),
};

static RIGHT_SENSOR: Sensor = Sensor {
	left_bot_offset: GameVec::new(PLAYER_SIZE.x, PLAYER_SIZE.y / 4),
	size: GameVec::new(PLAYER_SIZE.x + TILESIZE/2, PLAYER_SIZE.y * 3 / 4),
};

pub struct Player {
	pub left_bot: GameVec,
	pub velocity: GameVec,
	pub animation: Animation,
	walljumped: bool,
}

impl Player {
	pub fn new(left_bot: GameVec) -> Player {
		Player {
			left_bot,
			velocity: GameVec::new(0, 0),
			animation: Animation::new(AnimationId::BluePlayerIdle),
			walljumped: true,
		}
	}

	pub fn tick(&mut self, t: &TileMap, input: &dyn Input) {
		self.apply_forces(input, t);
		self.move_by_velocity(t);
	}

	fn apply_forces(&mut self, input: &dyn Input, t: &TileMap) {
		// drag
		if self.velocity.x.abs() < X_DRAG { self.velocity.x = 0; }
		else { self.velocity.x -= X_DRAG * self.velocity.x.signum(); }

		// walk
		self.velocity.x += input.horizontal_dir() * X_ACCELERATION;
		if self.velocity.x.abs() > MAX_X_VEL { self.velocity.x = MAX_X_VEL * self.velocity.x.signum(); }

		// jump
		if self.is_grounded(t) && input.up() && self.velocity.y <= 0 {
			self.velocity.y = JUMP_POWER;
			self.walljumped = false;
		}

		// walljump
		if !self.walljumped && !self.is_grounded(t) && input.up() && (
				self.is_left_walled(t) && input.horizontal_dir() > 0 ||
				self.is_right_walled(t) && input.horizontal_dir() < 0) {
			// let force = input.get_direction().normalize() * WALLJUMP_POWER as f32;
			// self.velocity = GameVec::new(force.x as i32, force.y as i32);
			// self.walljumped = true;
		}

		// gravity
		self.velocity.y -= GRAVITY;
	}

	fn is_grounded(&self, t: &TileMap) -> bool {
		self.check_sensor(&GROUND_SENSOR, t)
	}

	fn is_left_walled(&self, t: &TileMap) -> bool {
		self.check_sensor(&LEFT_SENSOR, t)
	}

	fn is_right_walled(&self, t: &TileMap) -> bool {
		self.check_sensor(&RIGHT_SENSOR, t)
	}
}
