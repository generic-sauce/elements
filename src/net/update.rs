use crate::prelude::*;

impl Packet for Update {}

#[derive(Serialize, Deserialize)]
pub struct Update {
	enemy_input_state: InputState,
	world_bytes: Vec<u8>,
}

impl Update {
	pub fn tuple(self) -> (InputState, World) {
		(self.enemy_input_state,
		 super::deser(&self.world_bytes[..]),
		)
	}

	pub fn from_tuple(enemy_input_state: InputState, world: &World) -> Update {
		Update {
			enemy_input_state,
			world_bytes: super::ser(world),
		}
	}
}
