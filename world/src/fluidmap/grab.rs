use crate::prelude::*;

const FLUID_GRAB_DIST: i32 = DESIRED_FLUID_DIST * 3 / 2;
const CURSOR_GRAB_DIST: i32 = TILESIZE * 4;

impl FluidMap {
	pub(in super) fn apply_grab(&self, mut f: Fluid, teams: &[u8], players: &[Player]) -> Fluid {
		for (i, player) in players.iter().enumerate() {
			if player.grab_cooldown.is_some() { continue; }

			let cursor = player.cursor_position();

			let condition = (cursor - f.position).as_short_as(CURSOR_GRAB_DIST) ||
				self.neighbours(&f)
					.any(|n|
						f.team == teams[i]
						&& (f.position - n.position).as_short_as(FLUID_GRAB_DIST)
							&& n.state == FluidState::AtHand(i as u8)
					);
			if condition {
				f.state = FluidState::AtHand(i as u8);
				break;
			}
		}

		f
	}
}
