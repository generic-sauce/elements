use crate::prelude::*;

impl Player {
	pub(in super) fn move_by_velocity(&mut self, t: &TileMap) {
		let mut remaining_vel = self.velocity;

		while remaining_vel != 0.into() {
			let xroute = route(remaining_vel.x, self.left_bot.x, PLAYER_SIZE.x);
			let yroute = route(remaining_vel.y, self.left_bot.y, PLAYER_SIZE.y);

			if xroute.abs() >= remaining_vel.x.abs() && yroute.abs() >= remaining_vel.y.abs() { // if no more collisions can happen!
				self.left_bot += remaining_vel;
				break;
			} else if (xroute * self.velocity.y).abs() < (yroute * self.velocity.x).abs() { //    <->    xroute / self.velocity.x < yroute / self.velocity.y    <->    xtime < ytime
				let ychange = xroute.abs() * remaining_vel.y / remaining_vel.x.abs();
				let change = Vec2i::new(xroute, ychange);

				let change_ex = change + (remaining_vel.x.signum(), remaining_vel.y.signum());
				if is_colliding(self.left_bot + change_ex, t) {
					assert!(!is_colliding(self.left_bot + change, t));

					remaining_vel -= change;
					self.left_bot += change;

					remaining_vel.x = 0;
					self.velocity.x = 0;
				} else {
					remaining_vel -= change_ex;
					self.left_bot += change_ex;
				}
			} else {
				let xchange = yroute.abs() * remaining_vel.x / remaining_vel.y.abs();
				let change = Vec2i::new(xchange, yroute);

				let change_ex = change + (remaining_vel.x.signum(), remaining_vel.y.signum());
				if is_colliding(self.left_bot + change_ex, t) {
					assert!(!is_colliding(self.left_bot + change, t));

					remaining_vel -= change;
					self.left_bot += change;

					remaining_vel.y = 0;
					self.velocity.y = 0;
				} else {
					remaining_vel -= change_ex;
					self.left_bot += change_ex;
				}
			}
		}
	}
}

fn is_colliding(left_bot: Vec2i, t: &TileMap) -> bool {
	let x_min = left_bot.x / TILESIZE;
	let x_max = (left_bot.x + PLAYER_SIZE.x - 1) / TILESIZE; // recall that PLAYER_SIZE = 1, means that x_min = x_max

	let y_min = left_bot.y / TILESIZE;
	let y_max = (left_bot.y + PLAYER_SIZE.y - 1) / TILESIZE;

	// TODO write this using .any()?
	for x in x_min..=x_max {
		for y in y_min..=y_max {
			if t.get((x as u32, y as u32).into()).is_solid() { return true; }
		}
	}
	false
}

// returns the change required to touch, but not collide the next tile
fn route(velocity: i32, min: i32, size: i32) -> i32 {
	if velocity < 0 {
		-(min % TILESIZE)
	} else {
		let max = min + size - 1;
		(TILESIZE-1) - (max % TILESIZE)
	}
}
