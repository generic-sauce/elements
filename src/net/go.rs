use crate::prelude::*;

impl Packet for Go {}

#[derive(Serialize, Deserialize)]
// TODO maybe add the tilemap
pub struct Go {
	pub your_player_id: usize,
}
