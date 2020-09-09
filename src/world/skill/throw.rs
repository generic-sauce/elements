use crate::prelude::*;

pub const GRAB_COOLDOWN: u32 = 10;

impl World {
	pub(in super) fn handle_throw(&mut self, p: usize) {
		self.fluidmap.iter_mut_notranslate()
			.filter(|x| x.owner == p && x.state == FluidState::AtHand)
			.for_each(|f| f.state = FluidState::Free);

		self.players[p].grab_cooldown = Some(GRAB_COOLDOWN);
	}

	pub(in super) fn handle_throw3(&mut self, p: usize) {
		let player = &self.players[p];
		let mut v: Vec<&mut Fluid> = self.fluidmap.iter_mut_notranslate()
			.filter(|x| x.owner == p && x.state == FluidState::AtHand)
			.collect();
		v.sort_by_cached_key(|f| throw_priority(f, player));
		if v.len() == 0 { return; }
		let best = v.pop().unwrap();
		v.sort_by_cached_key(|f| (f.position - best.position).length());
		v.truncate(2);

		let target_vel = best.velocity;

		let iter = Some(best).into_iter().chain(v.into_iter());
		for x in iter {
			x.state = FluidState::Free;
			x.ignore_counter = MAX_IGNORE_COUNTER;
			x.velocity = target_vel;
		}

		self.players[p].grab_cooldown = Some(GRAB_COOLDOWN);
	}
}

fn throw_priority(f: &Fluid, player: &Player) -> i32 {
	let relative_pos = f.position - player.center_position();
	relative_pos.dot(f.velocity)
}
