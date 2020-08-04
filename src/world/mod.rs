use core::default;

mod player;

pub struct World {
	pub players: Vec<Player>,
}

impl World {
	pub fn new() -> World {
		World {
			players: vec!(Player::new()),
		}
	}
}
