use crate::prelude::*;

impl Update {
	pub fn tuple(self) -> (u8, InputState, World) {
		(self.client_player_id,
		 self.server_input_state,
		 super::deser(&self.world_bytes[..]),
		)
	}

	pub fn from_tuple(client_player_id: u8, server_input_state: InputState, world: &World) -> Update {
		Update {
			client_player_id,
			server_input_state,
			world_bytes: super::ser(world),
		}
	}
}
