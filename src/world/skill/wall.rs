use crate::prelude::*;

const WALLS_PER_FLUID: u32 = 6;

impl World {
	pub(in super) fn handle_wall(&mut self, p: usize, handler: &mut impl EventHandler) {
		let player = &mut self.players[p];
		let cursor = player.cursor_position();
		if let Some(pos) = player.last_wall_pos {
			self.wall_from_to(p, pos, cursor, handler);
		} else {
			self.wall(p, cursor, handler);
		}
	}

	pub(in super) fn stop_wall(&mut self, p: usize) {
		self.players[p].last_wall_pos = None;
	}

	fn wall_from_to(&mut self, p: usize, from: GameVec, to: GameVec, handler: &mut impl EventHandler) {
		let n = (from - to).length() / TILESIZE * 8; // is this well?
		if n == 0 {
			self.wall(p, from, handler);
		} else {
			for i in 0..n {
				let current = from * (n-i-1) / (n-1) + to * i / (n-1);
				self.wall(p, current, handler);
			}
		}
	}

	fn wall(&mut self, p: usize, pos: GameVec, handler: &mut impl EventHandler) {
		let pos_tile: TileVec = pos.into();
		let tile = self.tilemap.get(pos_tile);

		let refill_amount = match tile {
			Tile::Void => WALL_LIFETIME,
			Tile::Wall { owner, remaining_lifetime } if owner == p => {
				WALL_LIFETIME - remaining_lifetime
			},
			_ => return,
		};

		if (0..2).any(|i| self.players[i].collides_tile(pos_tile)) {
			return;
		}

		if self.alloc_wall_lifetime(p, refill_amount).is_none() {
			self.players[p].last_wall_pos = None;
			return;
		}

		self.players[p].last_wall_pos = Some(pos);
		self.tilemap.set(pos_tile, Tile::Wall { owner: p, remaining_lifetime: WALL_LIFETIME });
		handler.tilemap_changed();
	}

	fn alloc_wall_lifetime(&mut self, p: usize, amount: u32) -> Option<()> {
		let mut pl = &mut self.players[p];

		// allocate free_wall
		if pl.free_wall_lifetime < amount {
			for inner_v in self.fluidmap.grid.iter_mut() {
				for _ in inner_v.drain_filter(|x| x.owner == p) {
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
}
