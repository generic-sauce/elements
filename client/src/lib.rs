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
				match self.socket.tick() {
					Some(GameSCPacket::Go { your_player_id, tilemap_image}) => {
						self.mode = ClientMode::InGame {
							player_id: your_player_id,
							world: World::new(0, &tilemap_image),
						};
					}
					Some(_) => println!("received non-Go packet while in ClientMode::Lobby"),
					None => {},
				}
			},
			ClientMode::InGame { player_id, world } => {
				// receive packets
				match self.socket.tick() {
					Some(GameSCPacket::WorldUpdate(update)) => apply_update_within_app(world, update, app),
					Some(_) => println!("received non-WorldUpdate packet while in ClientMode::InGame"),
					None => {},
				}

				// handle inputs
				if !app.menu.kind.is_active() {
					world.players[*player_id].input.update_gamepad(&app.input_backend.gamepad(0));
					world.players[*player_id].input.update_peripherals(&app.peripherals_state);
				} else {
					world.players[*player_id].input.clear();
				}

				// send packets
				self.socket.send(&GameCSPacket::InputState(world.players[*player_id].input.clone())).unwrap(); // TODO: fix clone

				// tick world
				tick_within_app(world, app);
			}
		}
	}

	pub fn draw(&mut self, app: &mut App<B>, draw: &mut Draw) {
		match &self.mode {
			ClientMode::Lobby => {
				draw_lobby::<B>(draw, &app.graphics_backend);
			},
			ClientMode::InGame { world, .. } => {
				draw_world(world, draw, app);
			}
		}
	}
}
