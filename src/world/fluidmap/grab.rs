use crate::prelude::*;

const FLUID_GRAB_DIST: i32 = 400 * 3 / 2;
const CURSOR_GRAB_DIST: i32 = TILESIZE * 4;

impl FluidMap {
	pub(in super) fn tick_grab(&mut self, players: &[Player; 2]) {
		for i in 0..self.grid.len() {
			let f = match &self.grid[i] {
				Some(x) => x,
				None => continue,
			};
			let player = &players[f.owner];
			if player.grab_cooldown.is_some() { continue; }

			let cursor = player.cursor_position();

			let condition = (cursor - f.position).as_short_as(CURSOR_GRAB_DIST) ||
				self.neighbours_with_owner(&f, FLUID_GRAB_DIST)
					.any(|n| n.state == FluidState::AtHand);

			if let Some(f) = &mut self.grid[i] {
				if condition {
					f.state = FluidState::AtHand;
				}
			}
		}
	}
}
