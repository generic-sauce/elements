use crate::prelude::*;

const FLUID_GRAB_DIST: i32 = DESIRED_FLUID_DIST * 3 / 2;
const CURSOR_GRAB_DIST: i32 = TILESIZE * 4;

impl FluidMap {
	pub(in super) fn apply_grab(&self, mut f: Fluid, players: &[Player; 2]) -> Fluid {
		let player = &players[f.owner];
		if player.grab_cooldown.is_some() { return f; }

		let cursor = player.cursor_position();

		let condition = (cursor - f.position).as_short_as(CURSOR_GRAB_DIST) ||
			self.neighbours_with_owner(&f)
				.any(|n|
					(f.position - n.position).as_short_as(FLUID_GRAB_DIST)
					&& matches!(n.state, FluidState::AtHand { .. })
				);
		if condition {
			f.state = FluidState::AtHand;
		}

		f
	}
}
