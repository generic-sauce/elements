use crate::prelude::*;

const GRAB_DIST: i32 = DESIRED_FLUID_DIST;

impl FluidMap {
	pub(in super) fn apply_grab(&self, mut f: Fluid, players: &[Player; 2]) -> Fluid {
		let player = &players[f.owner];
		if player.grab_cooldown.is_some() { return f; }

		let cursor = player.cursor_position();

		let condition = (cursor - f.position).as_short_as(GRAB_DIST) ||
			self.neighbours_with_owner(&f)
				.find(|n|
					(f.position - n.position).as_short_as(GRAB_DIST)
					&& n.state == FluidState::AtHand
				).is_some();
		if condition {
			f.state = FluidState::AtHand;
		}

		f
	}
}
