use crate::prelude::*;

#[allow(unused)]
pub fn send_packet(socket: &mut UdpSocket, p: &impl Packet) -> Result<(), SocketErr> {
	let packet_bytes = ser(p)?;
	let n: u32 = packet_bytes.len() as u32;
	let mut bytes = ser(&n)?;
	bytes.extend(packet_bytes);
	socket.send(&bytes[..])?;
	Ok(())
}

pub fn send_packet_to(socket: &mut UdpSocket, p: &impl Packet, target: SocketAddr) -> Result<(), SocketErr> {
	let packet_bytes = ser(p)?;
	let n: u32 = packet_bytes.len() as u32;
	let mut bytes = ser(&n)?;
	bytes.extend(packet_bytes);
	socket.send_to(&bytes[..], target)?;
	Ok(())
}

pub fn recv_packet<P: Packet>(socket: &mut UdpSocket) -> Result<Option<(P, SocketAddr)>, SocketErr> {
	let (bytes, addr) = match recv_bytes(socket)? {
		Some(x) => x,
		None => return Ok(None),
	};
	let p = deser::<P>(&bytes[..])?;
	Ok(Some((p, addr)))
}

pub fn recv_bytes(socket: &mut UdpSocket) -> Result<Option<(Vec<u8>, SocketAddr)>, SocketErr> {
	let mut n_bytes = [0u8; 4];
	assert_eq!(match socket.peek(&mut n_bytes[..]) {
		Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => return Ok(None),
		x=> x?,
	}, 4);
	let n: u32 = deser(&n_bytes[..])?;
	let mut bytes = vec![0u8; (n + 4) as usize];

	let (n_full, addr) = socket.recv_from(&mut bytes[..])?;

	assert_eq!(n_full, (n + 4) as usize);
	bytes.drain(..4);
	assert_eq!(bytes.len(), n as usize);
	Ok(Some((bytes, addr)))
}
