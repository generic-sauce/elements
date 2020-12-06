use crate::prelude::*;

impl Packet for InputState {}
impl Packet for WorldUpdate {}

#[derive(Serialize, Deserialize)]
pub struct Go {
	pub your_player_id: usize,
	pub tilemap_image: TileMapImage,
}

impl Packet for Go {}
