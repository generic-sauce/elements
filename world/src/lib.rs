#![feature(drain_filter)]

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
	pub players: Vec<Player>,
	pub teams: Vec<u8>, // teams[i] returns the team of the i'th player
	pub tilemap: TileMap,
	pub fluidmap: FluidMap,
	pub frame_id: u32,
	pub wins: [u32; 2], // number of wins, indexed by team 0 or team 1
	pub restart_state: RestartState,
	pub best_of_n: u32,
	pub bird: Animation,
}

pub enum GameResult {
	None,
	Winner(u8),
	Tie,
}

fn new_players(teams: &[u8], tilemap: &TileMap) -> Vec<Player> {
	let mut players = Vec::new();
    let mut next_spawn_position_index = [0, 0];

	for t in teams {
		let spawn_positions = tilemap.get_spawn_positions(*t);
		let spawn_position = spawn_positions[next_spawn_position_index[*t as usize] as usize];
		next_spawn_position_index[*t as usize] = (next_spawn_position_index[*t as usize] + 1) % spawn_positions.len();
		players.push(match t {
			0 => Player::new(spawn_position.into(), AnimationId::BluePlayerIdle, PlayerDirection::Right),
			1 => Player::new(spawn_position.into(), AnimationId::RedPlayerIdle, PlayerDirection::Left),
			_ => panic!("team out of range in new_players()"),
		});
	}

	players
}

impl World {
	pub fn reset(&mut self, handler: &mut impl EventHandler) {
		self.players = new_players(&self.teams[..], &self.tilemap);
		self.tilemap.reset(handler);
		self.fluidmap = FluidMap::new(self.tilemap.size);
		self.frame_id = 0;
		self.restart_state = RestartState::Game;
	}

	pub fn new(best_of_n: u32, tilemap_image: &TileMapImage, teams: &[u8]) -> World {
		let tilemap = TileMap::new(tilemap_image);

		World {
			players: new_players(teams, &tilemap),
			teams: teams.iter().cloned().collect(),
			fluidmap: FluidMap::new(tilemap.size),
			tilemap,
			frame_id: 0,
			wins: [0, 0],
			restart_state: RestartState::Game,
			best_of_n,
			bird: Animation::new(AnimationId::Bird),
		}
	}

	pub fn tick(&mut self, handler: &mut impl EventHandler) {
		self.bird.tick();

		let dead = [self.team_dead(0), self.team_dead(1)];
		// sub-tick
		match &mut self.restart_state {
			RestartState::Game => {
				self.tick_impl(handler);
				for team in 0..2 {
					if dead[team] {
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
					for team in 0..2 {
						if dead[team] {
							let winner_id = 1 - team;
							if matches!(*winner, Winner::One(_)) {
								*winner = Winner::Both;
							} else {
								*winner = Winner::One(winner_id as u32);
							}
							self.wins[winner_id as usize] += 1;
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
		for p in 0..self.players.len() {
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

		for i in 0..self.players.len() {
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
				state: FluidState::AtHand(i as u8),
				team: self.teams[i],
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
			if let Tile::Wall { remaining_lifetime, team } = tile {
				*tile = remaining_lifetime.checked_sub(1)
					.map(|lifetime| Tile::Wall { remaining_lifetime: lifetime, team: *team })
					.unwrap_or_else(|| { handler.tilemap_changed(); Tile::Void });
			}
		}
	}

	fn check_damage(&mut self, handler: &mut impl EventHandler) {
		for i in 0..self.players.len() {
			let team = self.teams[i];
			let mut dmg = 0;
			let t = &self.tilemap;
			let pl = &self.players;
			for v in self.fluidmap.grid.iter_mut() {
				dmg += v.drain_filter(|x|
					x.team != team && pl[i].collides_fluid(x.position, FLUID_DAMAGE_RADIUS, t)
				).map(|f| f.damage())
				.sum::<i32>();
			}
			if dmg > 0 {
				self.players[i].damage(dmg);
				handler.damage_inflicted(dmg, i);
			}
		}
	}

	pub fn team_dead(&self, team: u8) -> bool {
		self.players.iter()
			.enumerate()
			.filter(|(i, _)| self.teams[*i] == team)
			.all(|(_, x)| x.health == 0)
	}

	pub fn is_game_over(&self) -> GameResult {
		if self.best_of_n == 0 {
			return GameResult::None;
		};

		match self.restart_state {
			RestartState::Game => {
				let mut game_result = GameResult::None;
				for winner in self.wins.iter().enumerate().filter(|(_, win)| **win >= (self.best_of_n+1) / 2).map(|(index, _)| index) {
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
