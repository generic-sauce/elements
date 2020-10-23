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
	let packet_bytes = ser(p);
	let n: u32 = packet_bytes.len() as u32;
	let mut bytes = ser(&n);
	bytes.extend(packet_bytes);
	socket.write_message(bytes.into()).unwrap();
}

#[cfg(feature = "server")]
pub fn recv_packet<P: Packet>(socket: &mut TungSocket) -> Option<P> {
	if !socket.can_read() {
		return None;
	}

	let bytes = if let Message::Binary(b) = socket.read_message().unwrap() { b } else { panic!("non-binary message!"); };
	let p = deser::<P>(&bytes[4..]);
	Some(p)
}

pub fn ser<P: Serialize>(p: &P) -> Vec<u8> {
	serialize(p).unwrap()
}

pub fn deser<P: DeserializeOwned>(bytes: &[u8]) -> P {
	deserialize(bytes).unwrap()
}
