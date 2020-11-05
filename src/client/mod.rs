use crate::prelude::*;

pub enum ClientMode {
	Lobby,
	InGame { player_id: usize },
}

pub struct Client<B: Backend> {
	world: World,
	gamepad_state: RawGamepadState,
	socket: UdpSocket,
	phantom: PhantomData<B>,
	mode: ClientMode,
}

impl<B: Backend> Client<B> {
	pub fn new(server_ip: &str) -> Client<B> {
		let mut socket = UdpSocket::bind("0.0.0.0:0").expect("Could not create client socket");
		socket.set_nonblocking(true).unwrap();
		socket.connect((server_ip, PORT)).expect("Could not connect to server");

		send_packet(&mut socket, &Init::Init);

		Client {
			world: World::new(0),
			gamepad_state: RawGamepadState::new(),
			socket,
			phantom: PhantomData,
			mode: ClientMode::Lobby,
		}
	}

	pub fn tick(&mut self, app: &mut App<B>) {
		match self.mode {
			ClientMode::Lobby => {
				if let Some((Go { your_player_id }, _)) = recv_packet(&mut self.socket) {
					self.mode = ClientMode::InGame { player_id: your_player_id };
				}
			},
			ClientMode::InGame { player_id } => {
				// receive packets
				if let Some((update, _)) = recv_packet::<WorldUpdate>(&mut self.socket) {
					self.world.apply_update_within_app(update, app);
				}

				// handle inputs
				self.world.players[player_id].input.update_gamepad(&self.gamepad_state);
				self.world.players[player_id].input.update_peripherals(&app.peripherals_state);

				// send packets
				send_packet(&mut self.socket, &self.world.players[player_id].input);

				// tick world
				self.world.tick_within_app(app);
			}
		}
	}

	pub fn draw(&mut self, app: &mut App<B>) {
		match self.mode {
			ClientMode::Lobby => (), // TODO: drawing in lobby phase
			ClientMode::InGame { .. } => {
				let mut draw = Draw::new();
				self.world.draw(&mut draw);
				app.graphics_backend.draw(draw);
			}
		}
	}
}
