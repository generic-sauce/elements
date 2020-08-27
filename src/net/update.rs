use crate::prelude::*;

impl Update {
	pub fn tuple(self) -> (usize, InputState, World) {
		(self.your_player_id,
		 self.enemy_input_state,
		 super::deser(&self.world_bytes[..]),
		)
	}

	pub fn from_tuple(your_player_id: usize, enemy_input_state: InputState, world: &World) -> Update {
		Update {
			your_player_id,
			enemy_input_state,
			world_bytes: super::ser(world),
		}
	}
}
