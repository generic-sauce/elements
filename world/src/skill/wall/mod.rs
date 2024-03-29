mod pathfinder;

use crate::prelude::*;

const WALLS_PER_FLUID: u32 = 6;

impl World {
	pub(in super) fn handle_wall(&mut self, p: usize, handler: &mut impl EventHandler) {
		let cursor = self.players[p].cursor_position();

		let interpreted_cursor = self.interpret_wallpos(p, cursor);

		// deadzone
		let from = match self.players[p].wall_mode {
			WallMode::NoFluids => return,
			WallMode::NotWalling => interpreted_cursor,
			WallMode::InProgress { last_drawn_tile } => last_drawn_tile.to_game_center(),
		};

		self.wall_from_to(p, from, interpreted_cursor, handler);
	}

	pub(in super) fn stop_wall(&mut self, p: usize) {
		self.players[p].wall_mode = WallMode::NotWalling;
	}

	fn wall_from_to(&mut self, p: usize, from: GameVec, to: GameVec, handler: &mut impl EventHandler) {
		let path = self.generate_wall_path(from, to);
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
			Tile::Wall { team, remaining_lifetime } if team == self.teams[p] => {
				WALL_LIFETIME - remaining_lifetime
			},
			_ => {
				self.players[p].wall_mode = WallMode::InProgress { last_drawn_tile: pos_tile };
				return Some(())
			},
		};

		self.alloc_wall_lifetime(p, refill_amount)?;

		self.tilemap.set(pos_tile, Tile::Wall { team: self.teams[p], remaining_lifetime: WALL_LIFETIME });

		handler.tilemap_changed();
		self.players[p].wall_mode = WallMode::InProgress { last_drawn_tile: pos_tile };

		Some(())
	}

	fn alloc_wall_lifetime(&mut self, p: usize, amount: u32) -> Option<()> {
		let mut pl = &mut self.players[p];

		// allocate free_wall
		if pl.free_wall_lifetime < amount {
			'outer: for inner_v in self.fluidmap.grid.iter_mut() {
				while let Some(i) = inner_v.iter().position(|x| x.state == FluidState::AtHand(p as u8)) {
					inner_v.swap_remove(i);
					pl.free_wall_lifetime += WALLS_PER_FLUID * WALL_LIFETIME;
					if pl.free_wall_lifetime >= amount { break 'outer; }
				}
			}
		}

		if pl.free_wall_lifetime < amount {
			return None;
		}

		pl.free_wall_lifetime -= amount;
		Some(())
	}

	// yields what we expect the player meant by `start`
	// not guaranteed to be outside of any player
	fn interpret_wallpos(&self, p: usize, start: GameVec) -> GameVec {
		let pl = &self.players[p];
		let center = pl.center_position();

		let diff = start - center;

		// we don't unglitch if its too close the center of the player
		if diff.as_short_as(TILESIZE/2) {
			return start;
		}

		const STEP: i32 = 4;

		// binary search would be much faster - or one could even do it correctly ^^'
		(0..).map(|i| start + diff.with_length(i * STEP))
			.find(|pos| !pl.collides_tile(pos.to_tile()))
			.unwrap()
	}
}
