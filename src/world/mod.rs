pub mod player;
pub mod tilemap;
pub mod fluidmap;
mod draw;

use crate::prelude::*;

pub struct World {
	pub players: [Player; 2],
	pub tilemap: TileMap,
	pub fluidmap: FluidMap,
}

impl World {
	pub fn new() -> World {
		World {
			players: [Player::new(Vec2i::new(38 * TILESIZE, 45 * TILESIZE)), Player::new(Vec2i::new(64 * TILESIZE, 40 * TILESIZE))],
			tilemap: TileMap::new("res/map/map02.png"),
			fluidmap: FluidMap::new(),
		}
	}

	pub fn tick(&mut self, inputs: &mut [Box<dyn Input>; 2]) {
		self.fluidmap.tick(&self.tilemap);
		for (p, input) in self.players.iter_mut().zip(inputs.iter()) {
			p.tick(&self.tilemap, input.as_ref());
		}
	}
}
