mod pathfinder;

use crate::prelude::*;

const WALLS_PER_FLUID: u32 = 6;

impl World {
	pub(in super) fn handle_wall(&mut self, p: usize, handler: &mut impl EventHandler) {
		let player = &mut self.players[p];
		let cursor = player.cursor_position();

		// deadzone
		let too_short = (cursor - player.center_position()).as_short_as(TILESIZE/2);

		let (from, to) = match (player.wall_mode, too_short) {
			(WallMode::NoFluids, _) => return,
			(WallMode::NotWalling, _) => (cursor, cursor),
			(WallMode::InProgress { absolute, .. }, false) => (absolute, cursor),
			(WallMode::InProgress { absolute, relative }, true) => (absolute, player.center_position() + relative),
		};

		let relative = to - player.center_position();
		player.wall_mode = WallMode::InProgress { absolute: to, relative };

		self.wall_from_to(p, from, cursor, handler);
	}

	pub(in super) fn stop_wall(&mut self, p: usize) {
		self.players[p].wall_mode = WallMode::NotWalling;
	}

	fn wall_from_to(&mut self, p: usize, from: GameVec, to: GameVec, handler: &mut impl EventHandler) {
		let path = self.generate_wall_path(p, from, to);
		for t in path {
			if self.wall(p, t, handler).is_none() {
				self.players[p].wall_mode = WallMode::NoFluids;
				return;
			}
		}
	}

	fn wall(&mut self, p: usize, pos_tile: TileVec, handler: &mut impl EventHandler) -> Option<()> {
		assert!(!self.coll(pos_tile));

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

	// return a position close to pos, not colliding with the player p
	fn unglitch(&self, p: usize, start: GameVec) -> GameVec {
		let pl = &self.players[p];
		let center = pl.center_position();

		let mut diff = start - center;

		// in order to prevent division by zero
		if diff == v(0, 0) {
			diff = v(1, 0);
		}

		const STEP: i32 = 4;

		// binary search would be much faster - or one could even do it correctly ^^'
		(0..).map(|i| start + diff.with_length(i * STEP))
			.find(|pos| !pl.collides_tile(pos.to_tile()))
			.unwrap()
	}
}
