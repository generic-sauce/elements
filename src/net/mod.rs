mod update;

use crate::prelude::*;

pub trait Packet: Serialize + DeserializeOwned {}

#[derive(Serialize, Deserialize)]
pub struct Init;

#[derive(Serialize, Deserialize)]
pub struct Update {
	client_player_id: usize,
	server_input_state: InputState,
	world_bytes: Vec<u8>,
}

impl Packet for Init {}
impl Packet for Update {}
impl Packet for InputState {}

pub fn send_packet(socket: &mut UdpSocket, p: &impl Packet) {
	let bytes = ser(p);
	socket.send(&bytes[..]).unwrap();
}

pub fn send_packet_to(socket: &mut UdpSocket, p: &impl Packet, target: SocketAddr) {
	let bytes = ser(p);
	socket.send_to(&bytes[..], target).unwrap();
}

pub fn recv_packet<P: Packet>(socket: &mut UdpSocket) -> Option<(P, SocketAddr)> {
	let mut bytes = vec![0; 2000]; // TODO this may be a problem!
	let n = socket.recv(&mut bytes[..]).unwrap();
	bytes.truncate(n);

	deser(&bytes[..])
}


fn ser<P: Serialize>(p: &P) -> Vec<u8> {
	serialize(p).unwrap()
}

fn deser<P: DeserializeOwned>(bytes: &[u8]) -> P {
	deserialize(bytes).unwrap()
}
