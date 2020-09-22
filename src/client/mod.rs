use crate::prelude::*;

pub struct Client {
	app: App,
	input: InputDevice,
	socket: UdpSocket,
	player_id: usize,
}

impl Client {
	pub fn new(server_ip: &str) -> Client {
		let mut socket = UdpSocket::bind("0.0.0.0:0").expect("Could not create client socket");
		socket.set_nonblocking(true).unwrap();
		socket.connect((server_ip, PORT)).expect("Could not connect to server");

		send_packet(&mut socket, &Init::Init);

		let player_id = loop {
			// TODO maybe add a sleep here?
			if let Some((Go { your_player_id }, _)) = recv_packet::<Go>(&mut socket) {
				break your_player_id;
			}
		};

		let app = App::new();
		let input = InputDevice::new_adaptive(0, true, &app.gilrs);

		Client {
			app,
			input,
			socket,
			player_id,
		}
	}

	pub fn run(&mut self) {
		unimplemented!();
	}
}
