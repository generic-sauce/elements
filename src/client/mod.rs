use crate::prelude::*;

pub struct Client<B: Backend> {
	client_world: ClientWorld<B>,
	gamepad_state: RawGamepadState,
	socket: UdpSocket,
	player_id: usize,
}

impl<B: Backend> Client<B> {
	pub fn new(server_ip: &str) -> Client<B> {
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

impl<B: Backend> Runnable<B> for Client<B> {
	fn tick(&mut self, app: &mut App<B>) {
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

	fn draw(&mut self, app: &mut App<B>, timed_loop_info: &TimedLoopInfo) {
		let mut draw = Draw::new(timed_loop_info.elapsed_time);
		self.client_world.world.draw(&mut draw);
		app.graphics_backend.draw(draw);
	}

	fn get_runnable_change(&mut self) -> RunnableChange {
		RunnableChange::None
	}
}
