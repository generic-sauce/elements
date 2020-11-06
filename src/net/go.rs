use crate::prelude::*;

impl Packet for Go {}

#[derive(Serialize, Deserialize)]
pub struct Go {
	pub your_player_id: usize,
	pub tilemap_image: TileMapImage,
}
