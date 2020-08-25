use crate::prelude::*;

const WALLS_PER_FLUID: u32 = 4;

impl World {
	pub(in super) fn handle_wall(&mut self, p: usize) {
		// TODO decrease fluids count
		let player = &self.players[p];
		let cursor_tile: TileVec = player.cursor_position().into();
		let tile = self.tilemap.get(cursor_tile);

		if tile != Tile::Void { return; }
		if (0..2).any(|i| self.players[i].collides_tile(cursor_tile)) {
			return;
		}

		if self.alloc_wall(p).is_none() {
			return;
		}

		self.tilemap.set(cursor_tile, Tile::Wall { owner: p, remaining_lifetime: WALL_LIFETIME });
	}

	fn alloc_wall(&mut self, p: usize) -> Option<()> {
		let mut pl = &mut self.players[p];

        // allocate free_wall
		if pl.free_wall == 0 {
			for inner_v in self.fluidmap.grid.iter_mut() {
				if inner_v.drain_filter(|x| x.owner == p).next().is_some() {
					pl.free_wall += WALLS_PER_FLUID;
					break;
				}
			}
		}

		if pl.free_wall == 0 {
			return None;
		}

		pl.free_wall -= 1;
		return Some(());
	}
}