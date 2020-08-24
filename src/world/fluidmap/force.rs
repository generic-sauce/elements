use crate::prelude::*;

// if fluids have a distance <= FLUID_AFFECT_DIST they may affect each other
pub const FLUID_AFFECT_DIST: i32 = 500; // in game coordinates
const DESIRED_FLUID_DIST: i32 = 400;

const FLUID_GRAVITY: i32 = GRAVITY / 3;
const fn free_drag(x: i32) -> i32 { x * 255 / 256 }
const fn hand_drag(x: i32) -> i32 { x * 15 / 16 }

impl FluidMap {
	pub(in super) fn apply_forces<'a>(&'a self, t: &'a TileMap, players: &'a [Player; 2]) -> impl Iterator<Item=Fluid> + 'a {
		self.iter().map(move |f| {
			let mut f = f.clone();

			// gravity
			let mut velocity = f.velocity - GameVec::new(0, FLUID_GRAVITY);

			if let FluidState::AtHand = f.state {
				let cursor = players[f.owner].cursor_position();
				velocity += cursor_force(&f, cursor);
			}

			// drag
			let velocity = velocity.map(
				match f.state {
					FluidState::Free => free_drag,
					FluidState::AtHand => hand_drag,
				}
			);

			// position offset
			let neighbours: Vec<_> = self.neighbours(&f)
				.filter(|n| (f.position - n.position).as_short_as(DESIRED_FLUID_DIST))
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
				} ).sum::<GameVec>() / len * 0;

			f.velocity = velocity + velocity_update;
			f.move_and_slide(position_update, t);
			f
		})
	}
}

fn cursor_force(f: &Fluid, cursor: GameVec) -> GameVec {
    let v = cursor - f.position;
	(v / 4).length_clamped(42)
}
