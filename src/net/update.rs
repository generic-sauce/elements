use crate::prelude::*;

impl Packet for Update {}

#[derive(Serialize, Deserialize)]
pub struct Update {
	pub enemy_input_state: InputState,
	pub world_update: WorldUpdate,
}
