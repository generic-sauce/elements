pub mod player;
pub mod tilemap;
pub mod fluidmap;
pub mod skill;
mod draw;

use crate::prelude::*;

pub struct World {
	pub players: [Player; 2],
	pub tilemap: TileMap,
	pub fluidmap: FluidMap,
}

impl World {
	pub fn new() -> World {
		let tilemap = TileMap::new(&res("map/map02.png"));

		World {
			players: [
				Player::new(TileVec::new(37, 39).into(), PlayerDirection::Right, PlayerColor::Blue),
				Player::new(TileVec::new(88, 40).into(), PlayerDirection::Left, PlayerColor::Red)
			],
			fluidmap: FluidMap::new(tilemap.size),
			tilemap,
		}
	}

	pub fn tick(&mut self, inputs: &mut [Box<dyn Input>; 2], gilrs: &gilrs::Gilrs) {
		// sub-tick
		self.fluidmap.tick(&self.tilemap, &self.players);
		self.tick_players(inputs, gilrs);
		self.handle_skills(inputs);
		self.spawn_fluids();
		self.despawn_fluids();
		self.despawn_walls();
		self.check_damage();
	}

	fn tick_players(&mut self, inputs: &mut [Box<dyn Input>; 2], gilrs: &gilrs::Gilrs) {
		for p in 0..2 {
			inputs[p].update(&self.players[p], gilrs);
			self.tick_player(p, &mut *inputs[p]);
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
				id: self.fluidmap.next_id,
			});

			self.fluidmap.next_id = self.fluidmap.next_id.checked_add(1).unwrap_or(0);
		}
	}

	fn despawn_fluids(&mut self) {
		for cell in self.fluidmap.grid.iter_mut() {
			cell.drain_filter(|_| {
				let r = rand::random::<u32>() % 2000;
				r == 0
			});
		}
	}

	fn despawn_walls(&mut self) {
		let mut changed = false;
		for tile in self.tilemap.tiles.iter_mut() {
			if let Tile::Wall { remaining_lifetime, owner } = tile {
				*tile = remaining_lifetime.checked_sub(1)
					.map(|lifetime| Tile::Wall { remaining_lifetime: lifetime, owner: *owner })
					.unwrap_or_else(|| { changed = true; Tile::Void });
			}
		}

		if changed {
			self.tilemap.update_texture();
		}
	}

	fn check_damage(&mut self) {
		for i in 0..2 {
			let player = &mut self.players[i];
			let mut dmg = 0;
			for v in self.fluidmap.grid.iter_mut() {
				v.drain_filter(|x| x.owner != i && player.collides_point(x.position))
				 .for_each(|_| dmg += 5 )
			}
			if dmg > 0 { player.damage(dmg); }
		}
	}
}
