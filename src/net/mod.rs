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
	while socket.can_read() {
		let bytes = match socket.read_message() {
			Ok(Message::Binary(b)) => b,
			Ok(Message::Text(_)) => panic!("text should not be sent!"),
			Ok(_) => continue,
			Err(tungstenite::error::Error::Io(io_err)) => {
				if io_err.kind() == std::io::ErrorKind::WouldBlock {
					return None;
				}
				panic!("rcv error");
			}
			Err(_) => panic!("recv error"),
		};
		let p = deser::<P>(&bytes[..]);
		return Some(p);
	}
	None
}

pub fn ser<P: Serialize>(p: &P) -> Vec<u8> {
	serialize(p).unwrap()
}

pub fn deser<P: DeserializeOwned>(bytes: &[u8]) -> P {
	deserialize(bytes).unwrap()
}
