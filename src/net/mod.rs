#[cfg(feature = "master-server")] mod master;
#[cfg(feature = "master-server")] pub use master::*;
#[cfg(feature = "game")] mod game;
#[cfg(feature = "game")] pub use game::*;

use crate::prelude::*;

pub const DEFAULT_GAME_SERVER_PORT: u16 = 7575; // HTTP / UDP
pub const DEFAULT_GAME_SERVER_HTTPS_PORT: u16 = 7576; // HTTPS

pub const MASTER_SERVER_PORT: u16 = 7542;
pub const MASTER_SERVER_HTTPS_PORT: u16 = 7543;

pub trait Packet: Serialize + DeserializeOwned + Clone {}

#[derive(Serialize, Deserialize, Clone)]
pub enum MasterServerPacket {
	GameServerStatusUpdate {
		domain_name: String,
		num_players: u32,
		port: u16
	},
    ClientRequest { name: String },
}

impl Packet for MasterServerPacket {}

#[derive(Serialize, Deserialize, Clone)]
pub enum MasterClientPacket {
	GameRedirection(String, u16),
}

impl Packet for MasterClientPacket {}

#[allow(unused)]
pub fn send_packet(socket: &mut UdpSocket, p: &impl Packet) -> std::io::Result<()> {
	let packet_bytes = ser(p);
	let n: u32 = packet_bytes.len() as u32;
	let mut bytes = ser(&n);
	bytes.extend(packet_bytes);
	socket.send(&bytes[..])?;
	Ok(())
}

pub fn send_packet_to(socket: &mut UdpSocket, p: &impl Packet, target: SocketAddr) {
	let packet_bytes = ser(p);
	let n: u32 = packet_bytes.len() as u32;
	let mut bytes = ser(&n);
	bytes.extend(packet_bytes);
	socket.send_to(&bytes[..], target).unwrap();
}

pub fn recv_packet<P: Packet>(socket: &mut UdpSocket) -> Option<(P, SocketAddr)> {
	let (bytes, addr) = recv_bytes(socket)?;
	let p = deser::<P>(&bytes[..]);
	Some((p, addr))
}

pub fn recv_bytes(socket: &mut UdpSocket) -> Option<(Vec<u8>, SocketAddr)> {
	let mut n_bytes = [0u8; 4];
	assert_eq!(match socket.peek(&mut n_bytes[..]) {
		Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => return None,
		err => err.unwrap(),
	}, 4);
	let n: u32 = deser(&n_bytes[..]);
	let mut bytes = vec![0u8; (n + 4) as usize];

	let (n_full, addr) = socket.recv_from(&mut bytes[..]).unwrap();
	assert_eq!(n_full, (n + 4) as usize);
	bytes.drain(..4);
	assert_eq!(bytes.len(), n as usize);
	Some((bytes, addr))
}

pub fn ser<P: Serialize>(p: &P) -> Vec<u8> {
	serialize(p).unwrap()
}

pub fn deser<P: DeserializeOwned>(bytes: &[u8]) -> P {
	deserialize(bytes).unwrap()
}


// native

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

impl Packet for () {}