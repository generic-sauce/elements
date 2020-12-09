#[cfg(feature = "game")] mod game;

#[cfg(feature = "game")] pub use game::*;

use crate::prelude::*;

pub const PORT: u16 = 7575; // HTTP / UDP
pub const HTTPS_PORT: u16 = 7576; // HTTPS

pub trait Packet: Serialize + DeserializeOwned {}

#[derive(Serialize, Deserialize)]
// this is an enum as every socket object needs a size > 0
pub enum Init { Init }

impl Packet for Init {}

#[allow(unused)]
pub fn send_packet(socket: &mut UdpSocket, p: &impl Packet) {
	let packet_bytes = ser(p);
	let n: u32 = packet_bytes.len() as u32;
	let mut bytes = ser(&n);
	bytes.extend(packet_bytes);
	socket.send(&bytes[..]).unwrap();
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
