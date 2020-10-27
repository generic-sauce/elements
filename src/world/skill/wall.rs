use crate::prelude::*;

const WALLS_PER_FLUID: u32 = 6;

impl World {
	pub(in super) fn handle_wall(&mut self, p: usize, handler: &mut impl EventHandler) {
		let player = &mut self.players[p];
		let cursor = player.cursor_position();
		let from = match player.wall_mode {
			WallMode::NoFluids => return,
			WallMode::NotWalling => cursor,
			WallMode::InProgress(x) => x,
		};

		player.wall_mode = WallMode::InProgress(cursor);

		self.wall_from_to(p, from, cursor, handler);
	}

	pub(in super) fn stop_wall(&mut self, p: usize) {
		self.players[p].wall_mode = WallMode::NotWalling;
	}

	fn wall_from_to(&mut self, p: usize, mut current: GameVec, goal: GameVec, handler: &mut impl EventHandler) {
		loop {
			if self.wall_unglitched(p, self.unglitch(current), handler).is_none() {
				self.players[p].wall_mode = WallMode::NoFluids;
				return;
			}

			if self.unglitch(current) == self.unglitch(goal) {
				return; // we are done.
			}

			if let Some(neighbour) = (0..8000)
					.map(|i| current + (goal - current).with_length(i * TILESIZE/32))
					.find(|t| (self.unglitch(*t) - self.unglitch(current)).length_squared() == 1) {
				current = neighbour;
			} else {
				panic!("oh noes!");
			}

		}
	}

	// pos is guaranteed to not collide with a player
	fn wall_unglitched(&mut self, p: usize, pos_tile: TileVec, handler: &mut impl EventHandler) -> Option<()> {
		assert!((0..2).all(|i| !self.players[i].collides_tile(pos_tile)));

		let tile = self.tilemap.get(pos_tile);

		let refill_amount = match tile {
			Tile::Void => WALL_LIFETIME,
			Tile::Wall { owner, remaining_lifetime } if owner == p => {
				WALL_LIFETIME - remaining_lifetime
			},
			_ => return Some(()),
		};

		self.alloc_wall_lifetime(p, refill_amount)?;

		self.tilemap.set(pos_tile, Tile::Wall { owner: p, remaining_lifetime: WALL_LIFETIME });
		handler.tilemap_changed();

		Some(())
	}

	fn alloc_wall_lifetime(&mut self, p: usize, amount: u32) -> Option<()> {
		let mut pl = &mut self.players[p];

		// allocate free_wall
		if pl.free_wall_lifetime < amount {
			for inner_v in self.fluidmap.grid.iter_mut() {
				while let Some(i) = inner_v.iter().position(|x| x.owner == p) {
					inner_v.swap_remove(i);
					pl.free_wall_lifetime += WALLS_PER_FLUID * WALL_LIFETIME;
					if pl.free_wall_lifetime >= amount { break; }
				}
			}
		}

		if pl.free_wall_lifetime < amount {
			return None;
		}

		pl.free_wall_lifetime -= amount;
		Some(())
	}

	// return a position close to pos, not colliding with players
	fn unglitch(&self, mut pos: GameVec) -> TileVec {
		while let Some(p) = self.players.iter().find(|p| p.collides_tile(pos.to_tile())) {
			let center = p.center_position();
			let mut change = (pos - center).with_length(TILESIZE / 4);

			// preventing infinite loops: // TODO make more elegant
			if change.x == 0 { change.x = 5; }
			if change.y == 0 { change.y = 5; }

			pos += change;
		}

		pos.to_tile()
	}
}
