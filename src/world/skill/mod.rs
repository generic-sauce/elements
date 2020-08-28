use crate::prelude::*;

mod wall;

pub const GRAB_COOLDOWN: u32 = 10;

impl World {
	#[must_use]
	pub(in super) fn handle_skills(&mut self, inputs: &[InputState; 2]) -> Vec<Command> {
		let mut cmds = Vec::new();
		for p in 0..2 {
			if inputs[p].attack1 { self.handle_throw(p); }
			if inputs[p].attack2 { self.handle_throw3(p); }
			if inputs[p].special1 { cmds.extend(self.handle_wall(p)); }
			else { self.stop_wall(p); }
		}
		cmds
	}

	fn handle_throw(&mut self, p: usize) {
		self.fluidmap.iter_mut_notranslate()
			.filter(|x| x.owner == p)
			.for_each(|f| f.state = FluidState::Free);

		self.players[p].grab_cooldown = Some(GRAB_COOLDOWN);
	}

	fn handle_throw3(&mut self, p: usize) {
		self.fluidmap.iter_mut_notranslate()
			.filter(|x| x.owner == p)
			.take(3)
			.for_each(|f| f.state = FluidState::Free)
	}
}
