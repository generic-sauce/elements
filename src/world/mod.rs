pub mod player;
pub mod tilemap;
pub mod fluidmap;
pub mod skill;
mod update;
mod event;

pub use update::*;
pub use event::*;

use crate::prelude::*;

const FLUID_DAMAGE_RADIUS: i32 = TILESIZE * 3 / 2;

#[derive(Serialize, Deserialize)]
pub struct World {
	pub players: [Player; 2],
	pub tilemap: TileMap,
	pub fluidmap: FluidMap,
	pub frame_id: u32,
	pub kills: [u32; 2],
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
	}

	pub fn new() -> World {
		let tilemap = TileMap::new(&res("map/map02.png"));

		World {
			players: new_players(),
			fluidmap: FluidMap::new(tilemap.size),
			tilemap,
			frame_id: 0,
			kills: [0, 0],
		}
	}

	pub fn tick(&mut self, handler: &mut impl EventHandler) {
		// sub-tick
		self.tick_fluidmap();
		self.tick_players();
		self.handle_skills(handler);
		self.spawn_fluids();
		self.despawn_fluids();
		self.despawn_walls(handler);
		self.check_damage(handler);
		self.frame_id += 1;
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
				id: self.fluidmap.next_id,
			});

			self.fluidmap.next_id = self.fluidmap.next_id.checked_add(1).unwrap_or(0);
		}
	}

	fn despawn_fluids(&mut self) {
		for cell in self.fluidmap.grid.iter_mut() {
			cell.drain_filter(|f| f.check_despawn());
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
			let player = &mut self.players[i];
			let mut dmg = 0;
			for v in self.fluidmap.grid.iter_mut() {
				dmg += v.drain_filter(|x|
					x.owner != i && player.collides_point_with_radius(x.position, FLUID_DAMAGE_RADIUS)
				).map(|f| f.damage())
				.sum::<i32>();
			}
			if dmg > 0 {
				player.damage(dmg);
				handler.damage_inflicted(dmg, i);
			}
		}
	}

	pub fn player_dead(&self) -> Option<usize> {
		(0..2).find(|&p| self.players[p].health == 0)
	}
}
