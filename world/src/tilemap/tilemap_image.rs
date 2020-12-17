use crate::prelude::*;

pub type Rgba = [u8; 4];

#[derive(Serialize, Deserialize, Clone)]
pub struct TileMapImage {
	pub pixels: Vec<Vec<Rgba>>, // pixels[x][y]; pixels[0][0] is left-bot
	pub size: TileVec,
}

pub const DEFAULT_TILEMAP: &'static str = "map/map04.png";

