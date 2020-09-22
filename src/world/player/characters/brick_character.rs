use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct BrickCharacter {
}

impl BrickCharacter {
	pub fn new() -> BrickCharacter {
		BrickCharacter {
		}
	}

	pub fn tick(&mut self, p: usize, players: &mut [Player; 2], tilemap: &mut TileMap, Brickmap: &mut FluidMap) {
	}
}
