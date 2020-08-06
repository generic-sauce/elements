mod draw;
mod physics;
pub mod sensor;

use crate::prelude::*;

// The left-bottom of the tile (x,y) is located at position (x,y) * TILESIZE.
pub const TILESIZE: i32 = 256; // TODO move this where it belongs
pub const PLAYER_SIZE: Vec2i = Vec2i::new(2 * TILESIZE, 6 * TILESIZE);

static GROUND_SENSOR: Sensor = Sensor {
	left_bot: Vec2i::new(-10, 0),
	size: Vec2i::new(PLAYER_SIZE.x, 20),
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
		self.velocity.x += input.get_direction().x as i32 * 20;

		// jump
		if self.is_grounded(t) && input.get_direction().y > 0.0 && self.velocity.y <= 0 {
			self.velocity.y = 200;
		}

		// gravity
		self.velocity.y -= 2;

		// drag
		self.velocity -= self.velocity / 20;
	}

	fn is_grounded(&self, t: &TileMap) -> bool {
		self.check_sensor(&GROUND_SENSOR, t)
	}
}
