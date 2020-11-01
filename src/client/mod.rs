use crate::prelude::*;

pub struct Client {
	client_world: ClientWorld,
	gamepad_state: RawGamepadState,
	socket: UdpSocket,
	player_id: usize,
}

impl Client {
	pub fn new(server_ip: &str) -> Client {
		let mut socket = UdpSocket::bind("0.0.0.0:0").expect("Could not create client socket");
		socket.set_nonblocking(true).unwrap();
		socket.connect((server_ip, PORT)).expect("Could not connect to server");

		send_packet(&mut socket, &Init::Init);

		let player_id = TimedLoop::with_fps(10).filter_map(|_| {
			recv_packet::<Go>(&mut socket).map(|(go_packet, _)| go_packet.your_player_id)
		}).next().unwrap();

		Client {
			client_world: ClientWorld::new(0),
			gamepad_state: RawGamepadState::new(),
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
		self.client_world.world.players[self.player_id].input.update_gamepad(&self.gamepad_state);
		self.client_world.world.players[self.player_id].input.update_peripherals(&app.peripherals_state);

		// send packets
		send_packet(&mut self.socket, &self.client_world.world.players[self.player_id].input);

		// tick world
		self.client_world.tick(app);
	}

	fn draw(&mut self, app: &mut App, timed_loop_info: &TimedLoopInfo) {
		self.client_world.draw(app, timed_loop_info);
	}

	fn get_runnable_change(&mut self) -> RunnableChange {
		RunnableChange::None
	}
}
