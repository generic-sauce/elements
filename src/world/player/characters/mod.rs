mod fluid_character;
mod brick_character;

pub use fluid_character::*;
pub use brick_character::*;

use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub enum Character {
	FluidCharacter(FluidCharacter),
	BrickCharacter(BrickCharacter),
}

impl World {
	pub fn tick_character(&mut self, p: usize) {
		match &mut (self.characters[p]) {
			Character::FluidCharacter(c) => c.tick(p, &mut self.players, &mut self.tilemap, &mut self.fluidmap),
			Character::BrickCharacter(c) => c.tick(p, &mut self.players, &mut self.tilemap, &mut self.fluidmap),
		};
	}
}
