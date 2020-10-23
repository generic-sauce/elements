mod go;
pub use go::*;

use crate::prelude::*;

pub trait Packet: Serialize + DeserializeOwned {}

#[derive(Serialize, Deserialize)]
// this is an enum as every network object needs a size > 0
pub enum Init { Init }

impl Packet for Init {}
impl Packet for InputState {}
impl Packet for WorldUpdate {}

#[cfg(feature = "server")]
pub fn send_packet_to(socket: &mut TungSocket, p: &impl Packet) {
	socket.write_message(ser(p).into()).unwrap();
}

#[cfg(feature = "server")]
pub fn recv_packet<P: Packet>(socket: &mut TungSocket) -> Option<P> {
	if !socket.can_read() {
		return None;
	}

	let bytes = match socket.read_message().unwrap() {
		Message::Binary(b) => b,
		x => panic!("wrong message-format: {:?}", x),
	};
	let p = deser::<P>(&bytes[..]);
	Some(p)
}

pub fn ser<P: Serialize>(p: &P) -> Vec<u8> {
	serialize(p).unwrap()
}

pub fn deser<P: DeserializeOwned>(bytes: &[u8]) -> P {
	deserialize(bytes).unwrap()
}
