use crate::prelude::*;

impl Fluid {
	pub(in super) fn move_and_slide(&mut self, mut remaining_vel: GameVec, t: &TileMap) {
		if check_solid(self.position, self, t) {
			#[cfg(debug_assertions)]
			println!("A fluid is glitched.");
			return;
		}

		let reflect = |x: i32| -x / 3; // TODO add some randomness.

		while remaining_vel != 0.into() {
			let xroute = route(remaining_vel.x, self.position.x);
			let yroute = route(remaining_vel.y, self.position.y);

			let xroute_ex = xroute + remaining_vel.x.signum();
			let yroute_ex = yroute + remaining_vel.y.signum();

			if xroute.abs() >= remaining_vel.x.abs() && yroute.abs() >= remaining_vel.y.abs() { // if no more collisions can happen!
				self.position += remaining_vel;
				break;
			} else if (remaining_vel.y == 0 && yroute_ex == 0) /* edge case */ || (xroute_ex * remaining_vel.y).abs() < (yroute_ex * remaining_vel.x).abs() { //    <->    xroute / self.velocity.x < yroute / self.velocity.y    <->    xtime < ytime
				assert!(remaining_vel.x != 0);

				let ychange = xroute.abs() * remaining_vel.y / remaining_vel.x.abs();
				let change = GameVec::new(xroute, ychange);

				let change_ex = change + (remaining_vel.x.signum(), 0);
				if check_solid(self.position + change_ex, self, t) {
					assert!(!check_solid(self.position + change, self, t));

					remaining_vel -= change;
					self.position += change;

					remaining_vel.x = 0;
					self.velocity.x = reflect(self.velocity.x);
				} else {
					remaining_vel -= change_ex;
					self.position += change_ex;
				}
			} else {
				assert!(remaining_vel.y != 0);

				let xchange = yroute.abs() * remaining_vel.x / remaining_vel.y.abs();
				let change = GameVec::new(xchange, yroute);

				let change_ex = change + (0, remaining_vel.y.signum());
				if check_solid(self.position + change_ex, self, t) {
					assert!(!check_solid(self.position + change, self, t));

					remaining_vel -= change;
					self.position += change;

					remaining_vel.y = 0;
					self.velocity.y = reflect(self.velocity.y);
				} else {
					remaining_vel -= change_ex;
					self.position += change_ex;
				}
			}
		}
	}
}

// returns the change required to touch, but not collide the next tile
fn route(velocity: i32, pos: i32) -> i32 {
	if velocity < 0 {
		-(pos % TILESIZE)
	} else {
		(TILESIZE-1) - (pos % TILESIZE)
	}
}

fn check_solid(position: GameVec, f: &Fluid, t: &TileMap) -> bool {
	let tile = t.get(position.to_tile());
	if let Tile::Wall { owner, remaining_lifetime } = tile {
		if owner == f.owner
			&& f.state == FluidState::AtHand
			&& (WALL_LIFETIME - remaining_lifetime) <= WALL_IGNORE_FRIENDLY_FLUIDS_TIME {
				return false;
		}
	}
	t.check_solid(position)
}

impl FluidMap {
	pub(in super) fn move_fluid_by_velocity(mut f: Fluid, t: &TileMap) -> Fluid {
		f.move_and_slide(f.velocity, t);
		f
	}
}
