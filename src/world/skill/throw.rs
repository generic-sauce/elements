use crate::prelude::*;

pub const GRAB_COOLDOWN: u32 = 10;

impl World {
	pub(in super) fn handle_throw(&mut self, p: usize) {
		self.fluidmap.iter_mut_notranslate()
			.filter(|x| x.owner == p)
			.for_each(|f| f.state = FluidState::Free);

		self.players[p].grab_cooldown = Some(GRAB_COOLDOWN);
	}

	pub(in super) fn handle_throw3(&mut self, p: usize) {
		let player = &self.players[p];
		let mut v: Vec<&mut Fluid> = self.fluidmap.iter_mut_notranslate()
			.filter(|x| x.owner == p)
			.collect();
		v.sort_by_cached_key(|f| -throw_priority(f, player));
		v.truncate(3);
		if v.len() == 0 { return; }
		let target_vel = v.iter()
			.map(|x| x.velocity)
			.sum::<GameVec>() / (v.len() as i32);
		for x in v {
			x.state = FluidState::Free;
			x.velocity = target_vel;
		}

		self.players[p].grab_cooldown = Some(GRAB_COOLDOWN);
	}
}

fn throw_priority(f: &Fluid, player: &Player) -> i32 {
	let relative_pos = f.position - player.center_position();
	relative_pos.dot(f.velocity)
}