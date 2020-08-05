mod draw;

use crate::prelude::*;

// The left-bottom of the tile (x,y) is located at position (x,y) * TILESIZE.
pub const TILESIZE: i32 = 256; // TODO move this where it belongs
pub const PLAYER_SIZE: Vec2i = Vec2i::new(2 * TILESIZE, 6 * TILESIZE);

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

		// walk
		self.velocity.x += input.get_direction().x as i32 * 20;

		// jump
		if input.get_direction().y > 0.0 && self.velocity.y <= 0 {
			self.velocity.y = 200;
		}

		// gravity
		self.velocity.y -= 2;

		// drag
		self.velocity -= self.velocity / 20;
	}

	fn move_by_velocity(&mut self, t: &TileMap) {
		assert!(!is_colliding(self.left_bot, t));

		let old_lb = self.left_bot;
		let n = self.velocity.x.abs() + self.velocity.y.abs();

		for i in 0..n {
			let lb = old_lb + self.velocity * i / n;
			if is_colliding(lb, t) {
				self.velocity = 0.into(); // TODO nicely reduce velocity!
				break;
			}
			else { self.left_bot = lb; }
		}
	}

}

fn is_colliding(left_bot: Vec2i, t: &TileMap) -> bool {
	let x_min = left_bot.x / TILESIZE;
	let x_max = (left_bot.x + PLAYER_SIZE.x - 1) / TILESIZE; // recall that PLAYER_SIZE = 1, means that x_min = x_max

	let y_min = left_bot.y / TILESIZE;
	let y_max = (left_bot.y + PLAYER_SIZE.y - 1) / TILESIZE;

	// TODO write this using .any()?
	for x in x_min..=x_max {
		for y in y_min..=y_max {
			if t.get((x as u32, y as u32).into()).is_solid() { return true; }
		}
	}
	false
}
