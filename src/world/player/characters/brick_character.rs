use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct BrickCharacter {
	owns: Vec<TileVec>,
}

impl BrickCharacter {
	pub fn new() -> BrickCharacter {
		BrickCharacter {
			owns: Vec::<TileVec>::new(),
		}
	}

	pub fn tick(&mut self, p: usize, players: &mut [Player; 2], tilemap: &mut TileMap, fluidmap: &mut FluidMap) {
		self.spawn_brick(p, &mut players[p], tilemap, fluidmap);
	}

	pub fn spawn_brick(&mut self, p: usize, player: &mut Player, tilemap: &mut TileMap, fluidmap: &FluidMap) {
		let cursor = player.cursor_position();
	}
}
