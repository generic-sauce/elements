use crate::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub enum MasterServerPacket { // packets received by the master server
	GameServerStatusUpdate {
		domain_name: String,
		num_players: u32, // TODO this seems outdated
		port: u16
	},
	LoginRequest(/* username: */ String), // sent by a client to login to the master server; also used to rename yourself
	CreateLobby(/* lobby_name: */ String /*, settings */), // sent by a client to the master server to open a lobby (should only be sent, when you are not yet in a lobby)
	JoinLobby(/* lobby_id: */ u32),
	LeaveLobby, // sent from client to master server to indicate that it leaves it's lobby. this also closes the lobby if it was the last player
	LobbyListRequest, // sent from client to master server to receive a LobbyListResponsePacket
	StartGame, // sent from lobby owner to master server to indicate start of the game
	// ChangeLobbySettings(...), // sent from lobby owner to master server to change map/game-mode/...
}

impl Packet for MasterServerPacket {}

#[derive(Serialize, Deserialize, Clone)]
pub struct LobbyInfo {
	pub lobby_id: u32,
	pub name: String,
	/* tile_map */
	/* number_of_players */
	/* game_mode */
	/* ... */
}

#[derive(Serialize, Deserialize, Clone)]
pub enum MasterClientPacket { // packets sent from master server to client
	GoToGameServer(/* domain name */ String, /* port */ u16),
	LoginResponse, // master servers response to the LoginRequestPacket
	LobbyListResponse(Vec<LobbyInfo>), // sent from master server to client in order to inform about existing lobbies
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
