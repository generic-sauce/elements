mod fluid_character;

pub use fluid_character::*;

#[derive(Serialize, Deserialize, Clone)]
pub enum Character {
	FluidCharacter(FluidCharacter),
}
