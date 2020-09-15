use crate::prelude::*;

pub const GRAB_COOLDOWN: u32 = 30;
const THROW_THREE_DISTANCE: i32 = TILESIZE*2;

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
		v.push(best);

		let target_vel = v.iter().map(|x| x.velocity).sum::<GameVec>() / (v.len() as i32);

		for x in &mut v {
			x.state = FluidState::Free;
			x.ignore_counter = MAX_IGNORE_COUNTER;
			x.velocity = target_vel;
		}

		// TODO: check whether target position collides tilemap
		if v.len() >= 2 {
			v[1].position = v[0].position + (v[1].position - v[0].position).with_length(THROW_THREE_DISTANCE);
		}
		if v.len() >= 3 {
			let v0_to_v1 = v[1].position - v[0].position;
			let v0_to_v1_rotated = GameVec::new(v0_to_v1.y, -v0_to_v1.x);
			v[2].position = v[0].position + (v0_to_v1 / 2) + (v0_to_v1_rotated * 7 / 10);
		}

		self.players[p].grab_cooldown = Some(GRAB_COOLDOWN);
	}
}

fn throw_priority(f: &Fluid, player: &Player) -> i32 {
	let relative_pos = f.position - player.center_position();
	relative_pos.dot(f.velocity)
}
