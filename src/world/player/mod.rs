mod draw;

use crate::prelude::*;

pub struct Player {
	// position is the center of the player
	pub position: Vec2f,
	// size.x is the half width of the player and size.y is the half height of the player
	pub size: Vec2f,
	pub speed: Vec2f,
	pub direction: Vec2f,
}

impl Player {
	pub fn new(position: Vec2f) -> Player {
		Player {
			position,
			size: Player::get_size(),
			speed: Vec2f::new(0.0, 0.0),
			direction: Vec2f::new(0.0, 0.0),
		}
	}
    pub fn get_size() -> Vec2f {
        Vec2f::new(2.0, 2.0)
    }

	pub fn tick(&mut self) {
        self.speed *= 0.9;
		self.speed += self.direction * 0.05;
		self.position += self.speed;
	}
}
