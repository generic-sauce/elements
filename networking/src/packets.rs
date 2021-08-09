use crate::prelude::*;

// packages received by the master server
#[derive(Serialize, Deserialize, Clone)]
pub enum MasterServerPacket {
	GameServerStatusUpdate {
		domain_name: String,
		num_players: u32,
		port: u16
	},
	LoginRequestPacket { name: String }, // sent by a client to login to the master server; also used to rename yourself
	PlayerListRequestPacket,
}

impl Packet for MasterServerPacket {}

#[derive(Serialize, Deserialize, Clone)]
pub enum MasterClientPacket {
	GameRedirection(/* domain name */ String, /* port */ u16),
	LoginResponsePacket(/* session id */ u32), // master servers response to the LoginRequestPacket
	PlayerListResponsePacket(Vec<(/* username */ String, /* session id */ u32)>),
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
