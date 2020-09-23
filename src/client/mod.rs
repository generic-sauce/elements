use crate::prelude::*;

pub struct Client {
	client_world: ClientWorld,
	input: InputDevice,
	socket: UdpSocket,
	player_id: usize,
}

impl Client {
	pub fn new(server_ip: &str, gilrs: &Gilrs) -> Client {
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

		let input = InputDevice::new_adaptive(0, true, gilrs);

		Client {
			client_world: ClientWorld::new(),
			input,
			socket,
			player_id,
		}
	}
}

impl Runnable for Client {
	fn tick(&mut self, app: &mut App) {
		// receive packets
		if let Some((update, _)) = recv_packet::<WorldUpdate>(&mut self.socket) {
			self.client_world.apply_update(update, &mut app.sound_manager);
		}

		// handle inputs
		self.client_world.world.players[self.player_id].input = self.input.update(&app.gilrs);

		// send packets
		send_packet(&mut self.socket, &self.client_world.world.players[self.player_id].input);

		// tick world
		self.client_world.tick(app);
	}

	fn draw(&mut self, app: &mut App, timed_loop_info: &TimedLoopInfo) {
		self.client_world.draw(app, timed_loop_info);
	}
}
