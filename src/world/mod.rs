mod player;

use core::default;
use sfml::system::Vector2f;

use crate::world::player::Player;

pub struct World {
	pub players: [Player; 2],
}

impl World {
	pub fn new() -> World {
		World {
			players: [Player::new(Vector2f::new(0.0, 0.0)), Player::new(Vector2f::new(20.0, 0.0))],
		}
	}
}
