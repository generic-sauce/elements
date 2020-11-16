use crate::prelude::*;

pub enum ClientMode {
	Lobby,
	InGame {
		player_id: usize,
		world: World
	},
}

pub struct Client<B: Backend> {
	pub socket: B::SocketBackend,
	pub mode: ClientMode,
}

impl<B: Backend> Client<B> {
	pub fn new(server_ip: &str) -> Client<B> {
		Client {
			socket: B::SocketBackend::new(server_ip),
			mode: ClientMode::Lobby,
		}
	}

	pub fn tick(&mut self, app: &mut App<B>) {
		match &mut self.mode {
			ClientMode::Lobby => {
				if !self.socket.is_open() { return; }
				if let Some(Go { your_player_id, tilemap_image }) = self.socket.try_recv() {
					self.mode = ClientMode::InGame {
						player_id: your_player_id,
						world: World::new(0, &tilemap_image),
					};
				}
			},
			ClientMode::InGame { player_id, world } => {
				// receive packets
				if let Some(update) = self.socket.try_recv::<WorldUpdate>() {
					world.apply_update_within_app(update, app);
				}

				// handle inputs
				world.players[*player_id].input.update_gamepad(&app.input_backend.gamepad(0));
				world.players[*player_id].input.update_peripherals(&app.peripherals_state);

				// send packets
				self.socket.send(&world.players[*player_id].input);

				// tick world
				world.tick_within_app(app);
			}
		}
	}

	pub fn draw(&mut self, app: &mut App<B>) {
		let mut draw = Draw::new();

		match &self.mode {
			ClientMode::Lobby => {
				draw_lobby(&mut draw, &app.graphics_backend, app.timer.elapsed_ms() as f32);
			},
			ClientMode::InGame { world, .. } => {
				world.draw(&mut draw);
			}
		}

		app.graphics_backend.submit(draw);
	}
}
