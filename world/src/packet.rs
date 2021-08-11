use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub enum GameCSPacket { // Client -> Server
	InputState(InputState),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GameSCPacket { // Server -> Client
	WorldUpdate(WorldUpdate),
	Go {
		your_player_id: usize,
		tilemap_image: TileMapImage,
		teams: Vec<u8>,
	}
}

impl Packet for GameCSPacket {}
impl Packet for GameSCPacket {}

