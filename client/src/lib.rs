#![allow(incomplete_features)]
#![feature(generic_associated_types)]

pub mod prelude;
use crate::prelude::*;

mod app;
mod backend;
mod draw;
mod menu;
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
	pub active: bool,
}

impl<B: Backend> Client<B> {
	pub fn new(server_ip: &str, port: u16) -> Result<Client<B>, SocketErr> {
		Ok(Client {
			socket: B::SocketBackend::new(server_ip, port)?,
			mode: ClientMode::Lobby,
			active: false, // TODO actually set this thing to true sometimes
		})
	}

	pub fn tick(&mut self, app: &mut App<B>) {
		match &mut self.mode {
			ClientMode::Lobby => {
				self.socket.tick();
				match self.socket.recv() {
					Ok(Some(GameSCPacket::Go { your_player_id, tilemap_image, teams})) => {
						self.mode = ClientMode::InGame {
							player_id: your_player_id,
							world: World::new(0, &tilemap_image, &teams[..]),
						};
					}
					Ok(Some(_)) => println!("received non-Go packet while in ClientMode::Lobby"),
					Ok(None) => {},
					Err(x) => eprintln!("client::tick: lobby: can't socket.recv(): \"{}\"", x),
				}
			},
			ClientMode::InGame { player_id, world } => {
				// receive packets
				self.socket.tick();
				loop {
					match self.socket.recv() {
						Ok(Some(GameSCPacket::WorldUpdate(update))) => apply_update_within_app(world, update, app),
						Ok(Some(_)) => println!("received non-WorldUpdate packet while in ClientMode::InGame"),
						Ok(None) => break,
						Err(x) => eprintln!("client::tick: in-game: can't socket.recv(): \"{}\"", x),
					}
				}

				// handle inputs
				if !self.active {
					world.players[*player_id].input.update_gamepad(&app.input_backend.gamepad(0));
					world.players[*player_id].input.update_peripherals(&app.peripherals_state);
				} else {
					world.players[*player_id].input.clear();
				}

				// send packets
				if let Err(x) = self.socket.send(&GameCSPacket::InputState(world.players[*player_id].input.clone())) { // TODO: fix clone
					eprintln!("client: can't send GameCSPacket::InputState due to \"{}\"", x);
				}

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
