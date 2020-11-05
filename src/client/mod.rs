use crate::prelude::*;

pub enum ClientMode {
	Lobby,
	InGame { player_id: usize },
}

pub struct Client<B: Backend> {
	world: World,
	gamepad_state: RawGamepadState,
	socket: B::SocketBackend,
	mode: ClientMode,
}

impl<B: Backend> Client<B> {
	pub fn new(server_ip: &str) -> Client<B> {
		let socket = B::SocketBackend::new(server_ip);

		Client {
			world: World::new(0),
			gamepad_state: RawGamepadState::new(),
			socket,
			mode: ClientMode::Lobby,
		}
	}

	pub fn tick(&mut self, app: &mut App<B>) {
		match self.mode {
			ClientMode::Lobby => {
				if let Some(Go { your_player_id }) = self.socket.try_recv() {
					self.mode = ClientMode::InGame { player_id: your_player_id };
				}
			},
			ClientMode::InGame { player_id } => {
				// receive packets
				if let Some(update) = self.socket.try_recv::<WorldUpdate>() {
					self.world.apply_update_within_app(update, app);
				}

				// handle inputs
				self.world.players[player_id].input.update_gamepad(&self.gamepad_state);
				self.world.players[player_id].input.update_peripherals(&app.peripherals_state);

				// send packets
				self.socket.send(&self.world.players[player_id].input);

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
				app.graphics_backend.draw(draw, Some(&self.world));
			}
		}
	}
}
