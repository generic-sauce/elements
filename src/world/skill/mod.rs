use crate::prelude::*;

mod wall;
mod throw;

impl World {
	pub(in super) fn handle_skills(&mut self, handler: &mut impl EventHandler) {
		for p in 0..2 {
			if self.players[p].input.attack1 { self.handle_throw(p); }
			if self.players[p].input.attack2 { self.handle_throw3(p); }
			if self.players[p].input.special1 { self.handle_wall(p, handler); }
			else { self.stop_wall(p); }
		}
	}
}
