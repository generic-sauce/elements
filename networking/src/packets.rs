use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub enum MasterServerPacket { // packets received by the master server
	GameServerStatusUpdate {
		domain_name: String,
		num_players: u32, // TODO this seems outdated
		port: u16
	},
	Login(/* username: */ String), // sent by a client to login to the master server; also used to rename yourself
	CreateLobby(/* lobby_name: */ String), // sent by a client to the master server to open a lobby (should only be sent, when you are not yet in a lobby); the settings are set after creating the lobby
	JoinLobby(/* lobby_id: */ u32),
	LeaveLobby, // sent from client to master server to indicate that it leaves it's lobby. this also closes the lobby if it was the last player
	LobbyListRequest, // sent from client to master server to receive a LobbyListResponsePacket
	StartGame, // sent from lobby owner to master server to indicate start of the game
	// ChangeLobbySettings(...), // sent from lobby owner to master server to change map/game-mode/...
}

impl Packet for MasterServerPacket {}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShortLobbyInfo { // obtainable for each lobby
	pub lobby_id: u32,
	pub name: String,
	pub no_players: u32,
	pub max_no_players: u32,
	/* tile_map icon */
	/* number_of_players */
	/* game_mode */
	/* ... */
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LongLobbyInfo { // obtained only for the lobby where you are in
	pub lobby_id: u32,
	pub name: String,
	pub player_names: Vec<String>,
	pub your_player_index: u32,
	/* full tile_map */
	/* game_mode */
	/* ... */
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MasterClientPacket { // packets sent from master server to client
	GoToGameServer(/* domain name */ String, /* port */ u16),
	LobbyListResponse(Vec<ShortLobbyInfo>), // sent from master server to client in order to inform about existing lobbies
	LobbyInfoUpdate(LongLobbyInfo), // sent from master server to all clients within a lobby (when they create/join OR when some other player joins/leaves OR when a lobby setting is changed)
}

impl Packet for MasterClientPacket {}

// native udp packet wrapper

#[derive(Clone)]
pub enum NativeCSPacket<P: Packet> { // Native Client To Server Packet
	Payload(P),
	Heartbeat,
}

impl<P: Packet> Serialize for NativeCSPacket<P> {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where S: Serializer {
		match self {
			NativeCSPacket::Heartbeat => None,
			NativeCSPacket::Payload(p) => Some(p.clone()),
		}.serialize(serializer)
	}
}

impl<'de, P: Packet> Deserialize<'de> for NativeCSPacket<P> {
	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where D: Deserializer<'de> {
		<Option<P>>::deserialize(deserializer)
			.map(|opt| match opt {
				None => NativeCSPacket::Heartbeat,
				Some(p) => NativeCSPacket::Payload(p),
			})
	}
}

impl<P: Packet> Packet for NativeCSPacket<P> {}
