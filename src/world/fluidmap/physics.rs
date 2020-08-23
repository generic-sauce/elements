use crate::prelude::*;

impl FluidMap {
	pub(in super) fn move_fluid_by_velocity(mut f: Fluid, t: &TileMap) -> Fluid {
		let mut remaining_vel = f.velocity;

		while remaining_vel != 0.into() {
			let xroute = route(remaining_vel.x, f.position.x);
			let yroute = route(remaining_vel.y, f.position.y);

			let xroute_ex = xroute + remaining_vel.x.signum();
			let yroute_ex = yroute + remaining_vel.y.signum();

			if xroute.abs() >= remaining_vel.x.abs() && yroute.abs() >= remaining_vel.y.abs() { // if no more collisions can happen!
				f.position += remaining_vel;
				break;
			} else if (remaining_vel.y == 0 && yroute_ex == 0) /* edge case */ || (xroute_ex * remaining_vel.y).abs() < (yroute_ex * remaining_vel.x).abs() { //    <->    xroute / self.velocity.x < yroute / self.velocity.y    <->    xtime < ytime
				#[cfg(debug_assertions)]
				assert!(remaining_vel.x != 0);

				let ychange = xroute.abs() * remaining_vel.y / remaining_vel.x.abs();
				let change = GameVec::new(xroute, ychange);

				let change_ex = change + (remaining_vel.x.signum(), 0);
				if t.check_solid(f.position + change_ex) {
					#[cfg(debug_assertions)]
					assert!(!t.check_solid(f.position + change));

					remaining_vel -= change;
					f.position += change;

					remaining_vel.x = 0;
					f.velocity.x = 0;
				} else {
					remaining_vel -= change_ex;
					f.position += change_ex;
				}
			} else {
				#[cfg(debug_assertions)]
				assert!(remaining_vel.y != 0);

				let xchange = yroute.abs() * remaining_vel.x / remaining_vel.y.abs();
				let change = GameVec::new(xchange, yroute);

				let change_ex = change + (0, remaining_vel.y.signum());
				if t.check_solid(f.position + change_ex) {
					#[cfg(debug_assertions)]
					assert!(!t.check_solid(f.position + change));

					remaining_vel -= change;
					f.position += change;

					remaining_vel.y = 0;
					f.velocity.y = 0;
				} else {
					remaining_vel -= change_ex;
					f.position += change_ex;
				}
			}
		}

		f
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

