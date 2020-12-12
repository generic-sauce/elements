use crate::prelude::*;

// if fluids have a distance <= FLUID_AFFECT_DIST they may affect each other
pub const FLUID_AFFECT_DIST: i32 = 500; // in game coordinates
pub const DESIRED_FLUID_DIST: i32 = 400;

const FLUID_GRAVITY: i32 = GRAVITY / 3;
const fn free_drag(x: i32) -> i32 { x * 255 / 256 }
const fn hand_drag(x: i32) -> i32 { x * 24 / 32 }

impl FluidMap {
	pub(in super) fn apply_forces(&self, mut f: Fluid, t: &TileMap, players: &[Player; 2], frame_id: u32) -> Fluid {
		// gravity
		let mut velocity = f.velocity - GameVec::new(0, FLUID_GRAVITY);

		if let FluidState::AtHand = f.state {
			let cursor = players[f.owner].cursor_position();
			apply_cursor_steering(&mut velocity, &f, cursor);
		}

		// drag
		let velocity = velocity.map(
			match f.state {
				FluidState::Free => free_drag,
				FluidState::AtHand => hand_drag,
			}
		);

		// noise
		let velocity = velocity + (
			noise(frame_id, f.id, 0),
			noise(frame_id, f.id, 1),
		);

		// position offset
		let neighbours: Vec<_> = self.neighbours_with_owner(&f)
			.filter(|n| ((n.ignore_counter == 0 && f.ignore_counter == 0) || f.id == n.id) && (f.position - n.position).as_short_as(DESIRED_FLUID_DIST))
			.collect();

		let len = neighbours.len().max(1) as i32;

		let position = neighbours.iter()
			.map(|n| (f.position - n.position).with_length(DESIRED_FLUID_DIST) + n.position )
			.sum::<GameVec>() / len;

		let position_update = position - f.position;

		let velocity_update = neighbours.iter()
			.map(|n| {
				let relative_velocity = n.velocity - velocity;
				let from_n = f.position - n.position;
				let projected = relative_velocity.projected_on(from_n);
				if projected.dot(from_n) < 0 {
					return GameVec::new(0, 0);
				}
				projected / 2
			} ).sum::<GameVec>() / len * 5 / 2;

		f.velocity = velocity + velocity_update;
		f.move_and_slide(position_update, t);

		f
	}
}

fn apply_cursor_steering(velocity: &mut GameVec, f: &Fluid, cursor: GameVec) {
	const MAX_SPEED: i32 = 600;
	const MAX_FORCE: i32 = 260;

	let desired_velocity = (cursor - f.position).length_clamped(MAX_SPEED);
	let steering = (desired_velocity - f.velocity)
		.length_clamped(MAX_FORCE);
	*velocity = (*velocity + steering).length_clamped(MAX_SPEED);
}

// returns -1 or 1
fn noise(fluid_id: u32, frame_id: u32, num: u32) -> i32 {
	match rng(1, 1, [fluid_id, num, frame_id, 0]) {
		true => 1,
		false => -1,
	}
}
