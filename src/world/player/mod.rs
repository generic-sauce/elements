mod draw;

use crate::prelude::*;

// The left-bottom of the tile (x,y) is located at position (x,y) * TILESIZE.
pub const TILESIZE: i32 = 256; // TODO move this where it belongs
pub const PLAYER_SIZE: Vec2i = Vec2i::new(2 * TILESIZE, 3 * TILESIZE); // TODO set correct player size

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
		self.apply_forces(input);
		self.move_by_velocity(t);
	}

	fn apply_forces(&mut self, input: &dyn Input) {
		if !input.is_connected() { println!("joystick not connected"); }
		self.velocity += input.get_direction().to_i() * 20;
		self.velocity.y -= 2;
	}

	fn move_by_velocity(&mut self, t: &TileMap) {
		self.velocity -= self.velocity / 10;
		self.left_bot += self.velocity;
	}
}
