#![feature(drain_filter)]
#![feature(const_fn_trait_bound)]

mod animation;
mod vec;
mod input;
mod rng;
mod resource;
mod player;
mod tilemap;
mod fluidmap;
mod skill;
mod event;
mod update;
mod packet;

pub mod prelude;
use crate::prelude::*;

pub const FIGHT_END_COUNT: u32 = 120;
pub const TROPHY_END_COUNT: u32 = 160;
const FLUID_DAMAGE_RADIUS: i32 = TILESIZE * 3 / 2;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Winner {
	None,
	Both,
	One(u32),
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum RestartState {
	Game,
	Restart { counter: u32, tick_value: f32, winner: Winner },
}

impl RestartState {
	pub fn new_restart() -> RestartState {
		RestartState::Restart { counter: 0, tick_value: 1.0, winner: Winner::None }
	}
}

pub fn get_frame_tick_probability(counter: u32) -> f32 {
	1.0 - counter as f32 / FIGHT_END_COUNT as f32
}

#[derive(Serialize, Deserialize)]
pub struct World {
	pub players: [Player; 2],
	pub tilemap: TileMap,
	pub fluidmap: FluidMap,
	pub frame_id: u32,
	pub kills: [u32; 2],
	pub restart_state: RestartState,
	pub best_of_n: u32,
}

pub enum GameResult {
	None,
	Winner(u8),
	Tie,
}

fn new_players() -> [Player; 2] {
	[
		Player::new(TileVec::new(37, 39).into(), AnimationId::BluePlayerIdle, PlayerDirection::Right),
		Player::new(TileVec::new(88, 40).into(), AnimationId::RedPlayerIdle, PlayerDirection::Left),
	]
}

impl World {
	pub fn reset(&mut self, handler: &mut impl EventHandler) {
		self.players = new_players();
		self.tilemap.reset(handler);
		self.fluidmap = FluidMap::new(self.tilemap.size);
		self.frame_id = 0;
		self.restart_state = RestartState::Game;
	}

	pub fn new(best_of_n: u32, tilemap_image: &TileMapImage) -> World {
		let tilemap = TileMap::new(tilemap_image);

		World {
			players: new_players(),
			fluidmap: FluidMap::new(tilemap.size),
			tilemap,
			frame_id: 0,
			kills: [0, 0],
			restart_state: RestartState::Game,
			best_of_n
		}
	}

	pub fn tick(&mut self, handler: &mut impl EventHandler) {
		// sub-tick
		match &mut self.restart_state {
			RestartState::Game => {
				self.tick_impl(handler);
				for player_dead in &self.player_dead() {
					if *player_dead {
						self.restart_state = RestartState::new_restart();
						handler.game_ended();
					}
				}
				self.frame_id += 1;
			},
			RestartState::Restart { counter, tick_value, winner } => {
				*counter += 1;
				*tick_value += get_frame_tick_probability(*counter);
				if *counter == FIGHT_END_COUNT {
					for i in 0..self.players.len() {
						if self.players[i].health == 0 {
							let winner_id = 1 - i;
							if matches!(*winner, Winner::One(_)) {
								*winner = Winner::Both;
							} else {
								*winner = Winner::One(winner_id as u32);
							}
							self.kills[winner_id] += 1;
						}
					}
				} else if *counter > TROPHY_END_COUNT {
					if self.players.iter().any(|p| p.input.restart()) {
						self.reset(handler);
						self.restart_state = RestartState::Game;
						handler.new_game_started();
					}
				} else if *counter > FIGHT_END_COUNT {

				} else if *tick_value >= 1.0 {
					*tick_value -= 1.0;
					self.tick_impl(handler);
				}
			}
		}
	}

	fn tick_impl(&mut self, handler: &mut impl EventHandler) {
		self.tick_fluidmap();
		self.tick_players();
		self.handle_skills(handler);
		self.spawn_fluids();
		self.despawn_fluids();
		self.despawn_walls(handler);
		self.check_damage(handler);
	}

	fn tick_players(&mut self) {
		for p in 0..2 {
			self.tick_player(p);
		}
	}

	fn spawn_fluids(&mut self) {
		if self.fluidmap.spawn_counter > 0 {
			self.fluidmap.spawn_counter -= 1;
			return;
		} else {
			self.fluidmap.spawn_counter = FLUID_SPAWN_DIST;
		}

		for i in 0..2 {
			let p = &self.players[i];

			let calc_spawn_pos = |from: GameVec, to: GameVec| {
				let accuracy = |v: GameVec| (v.x.abs() + v.y.abs()) / 40 + 2; // TODO is this a good choice?
				let n = accuracy(from - to);
				for i in 0..n {
					let current = from * (n-1-i) / (n-1) + to * i / (n-1);
					if !self.tilemap.check_solid(current) { return current; }
				}
				panic!("this implies that the player is glitched actually!");
			};

			let position = calc_spawn_pos(p.cursor_position(), p.center_position());

			self.fluidmap.add_fluid(Fluid {
				state: FluidState::AtHand,
				owner: i,
				velocity: 0.into(),
				position,
				reference_position: position,
				ignore_counter: 0,
				id: self.fluidmap.next_id,
			});

			self.fluidmap.next_id = self.fluidmap.next_id.checked_add(1).unwrap_or(0);
		}
	}

	fn despawn_fluids(&mut self) {
		let frame_id = self.frame_id;
		for cell in self.fluidmap.grid.iter_mut() {
			cell.drain_filter(|f| f.check_despawn(frame_id));
		}
	}

	fn despawn_walls(&mut self, handler: &mut impl EventHandler) {
		for tile in self.tilemap.tiles.iter_mut() {
			if let Tile::Wall { remaining_lifetime, owner } = tile {
				*tile = remaining_lifetime.checked_sub(1)
					.map(|lifetime| Tile::Wall { remaining_lifetime: lifetime, owner: *owner })
					.unwrap_or_else(|| { handler.tilemap_changed(); Tile::Void });
			}
		}
	}

	fn check_damage(&mut self, handler: &mut impl EventHandler) {
		for i in 0..2 {
			let mut dmg = 0;
			let t = &self.tilemap;
			let pl = &self.players;
			for v in self.fluidmap.grid.iter_mut() {
				dmg += v.drain_filter(|x|
					x.owner != i && pl[i].collides_fluid(x.position, FLUID_DAMAGE_RADIUS, t)
				).map(|f| f.damage())
				.sum::<i32>();
			}
			if dmg > 0 {
				self.players[i].damage(dmg);
				handler.damage_inflicted(dmg, i);
			}
		}
	}

	pub fn player_dead(&self) -> [bool; 2] {
		[self.players[0].health == 0, self.players[1].health == 0]
	}

	pub fn is_game_over(&self) -> GameResult {
		if self.best_of_n == 0 {
			return GameResult::None;
		};

		match self.restart_state {
			RestartState::Game => {
				let mut game_result = GameResult::None;
				for winner in self.kills.iter().enumerate().filter(|(_, kill)| **kill >= (self.best_of_n+1) / 2).map(|(index, _)| index) {
					match game_result {
						GameResult::None => { game_result = GameResult::Winner(winner as u8)},
						GameResult::Winner(_) => { game_result = GameResult::Tie },
						GameResult::Tie => {},
					}
				}
				game_result
			},
			_ => GameResult::None,
		}
	}
}
