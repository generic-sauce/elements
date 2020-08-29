mod update;
mod go;

pub use update::*;
pub use go::*;

use crate::prelude::*;

pub const PORT: u16 = 7575;
const PACKET_SIZE: usize = 20000;

pub trait Packet: Serialize + DeserializeOwned {}

#[derive(Serialize, Deserialize)]
// this is an enum as every network object needs a size > 0
pub enum Init { Init }

impl Packet for Init {}
impl Packet for InputState {}

#[allow(unused)]
pub fn send_packet(socket: &mut UdpSocket, p: &impl Packet) {
	let bytes = ser(p);
	assert!(bytes.len() <= PACKET_SIZE);
	socket.send(&bytes[..]).unwrap();
}

pub fn send_packet_to(socket: &mut UdpSocket, p: &impl Packet, target: SocketAddr) {
	let bytes = ser(p);
	assert!(bytes.len() <= PACKET_SIZE);
	socket.send_to(&bytes[..], target).unwrap();
}

pub fn recv_packet<P: Packet>(socket: &mut UdpSocket) -> Option<(P, SocketAddr)> {
	let mut bytes = vec![0; PACKET_SIZE]; // TODO this may be a problem!
	let (n, addr) = match socket.recv_from(&mut bytes[..]) {
		Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => return None,
		err => err.unwrap(),
	};
	let p = deser::<P>(&bytes[..n]);
	Some((p, addr))
}

fn ser<P: Serialize>(p: &P) -> Vec<u8> {
	serialize(p).unwrap()
}

fn deser<P: DeserializeOwned>(bytes: &[u8]) -> P {
	deserialize(bytes).unwrap()
}
