mod draw;
mod physics;
pub mod sensor;

use crate::prelude::*;

// The left-bottom of the tile (x,y) is located at position (x,y) * TILESIZE.
pub const TILESIZE: i32 = 256; // TODO move this where it belongs
pub const PLAYER_SIZE: Vec2i = Vec2i::new(2 * TILESIZE, 6 * TILESIZE);

const X_DRAG: i32 = 50;
const MAX_X_VEL: i32 = 500;
const JUMP_POWER: i32 = 1000;
const GRAVITY: i32 = 160;
const X_ACCELERATION: i32 = 200;

static GROUND_SENSOR: Sensor = Sensor {
	left_bot_offset: Vec2i::new(0, -1),
	size: Vec2i::new(PLAYER_SIZE.x, 2),
};

pub struct Player {
	pub left_bot: Vec2i,
	pub velocity: Vec2i,
}

impl Player {
	pub fn new(left_bot: Vec2i) -> Player {
		Player {
			left_bot,
			velocity: Vec2i::new(0, 0),
		}
	}

	pub fn tick(&mut self, t: &TileMap, input: &dyn Input) {
		self.apply_forces(input, t);
		self.move_by_velocity(t);
	}

	fn apply_forces(&mut self, input: &dyn Input, t: &TileMap) {
		if !input.is_connected() { println!("joystick not connected"); }

		// walk
		self.velocity.x += (input.get_direction().x * X_ACCELERATION as f32) as i32;
		if self.velocity.x.abs() > MAX_X_VEL {
			self.velocity.x = MAX_X_VEL * self.velocity.x.signum();
		}

		// jump
		if self.is_grounded(t) && input.get_direction().y > 0.0 && self.velocity.y <= 0 {
			self.velocity.y = JUMP_POWER;
		}

		// gravity
		self.velocity.y -= GRAVITY;

		// drag
		if self.velocity.x.abs() < X_DRAG { self.velocity.x = 0; }
		else { self.velocity.x -= X_DRAG * self.velocity.x.signum(); }
	}

	fn is_grounded(&self, t: &TileMap) -> bool {
		self.check_sensor(&GROUND_SENSOR, t)
	}
}
