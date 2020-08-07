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
		let tilemap = TileMap::new("res/map/map02.png");
		World {
			players: [Player::new(TileVec::new(38, 45).into()), Player::new(TileVec::new(64, 40).into())],
			fluidmap: FluidMap::new(tilemap.size),
			tilemap,
		}
	}

	pub fn tick(&mut self, inputs: &mut [Box<dyn Input>; 2]) {
		self.fluidmap.tick(&self.tilemap);
		for (p, input) in self.players.iter_mut().zip(inputs.iter()) {
			p.tick(&self.tilemap, input.as_ref());
		}
	}
}
