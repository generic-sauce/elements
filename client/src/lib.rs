#![allow(incomplete_features)]
#![feature(generic_associated_types)]

pub mod prelude;
use crate::prelude::*;

mod app;
mod backend;
mod draw;
mod menu;
mod server_connector;
mod local;


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
	pub fn new(server_ip: &str, port: u16) -> Client<B> {
		Client {
			socket: B::SocketBackend::new(server_ip, port),
			mode: ClientMode::Lobby,
		}
	}

	pub fn tick(&mut self, app: &mut App<B>) {
		match &mut self.mode {
			ClientMode::Lobby => {
				if !self.socket.is_open() { return; }
				if let Some(Go { your_player_id, tilemap_image }) = self.socket.tick() {
					self.mode = ClientMode::InGame {
						player_id: your_player_id,
						world: World::new(0, &tilemap_image),
					};
				}
			},
			ClientMode::InGame { player_id, world } => {
				// receive packets
				if let Some(update) = self.socket.tick::<WorldUpdate>() {
					apply_update_within_app(world, update, app);
				}

				// handle inputs
				world.players[*player_id].input.update_gamepad(&app.input_backend.gamepad(0));
				world.players[*player_id].input.update_peripherals(&app.peripherals_state);

				// send packets
				self.socket.send(&world.players[*player_id].input).unwrap();

				// tick world
				tick_within_app(world, app);
			}
		}
	}

	pub fn draw(&mut self, app: &mut App<B>) {
		let mut draw = Draw::new();

		match &self.mode {
			ClientMode::Lobby => {
				draw_lobby::<B>(&mut draw, &app.graphics_backend);
			},
			ClientMode::InGame { world, .. } => {
				draw_world(world, &mut draw, app);
			}
		}

		app.graphics_backend.submit(draw);
	}
}
