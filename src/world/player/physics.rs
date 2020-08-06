use crate::prelude::*;

impl Player {
	pub(in super) fn move_by_velocity(&mut self, t: &TileMap) {
		let mut remaining_vel = self.velocity;

		while remaining_vel != 0.into() {
			let xdist = dist(remaining_vel.x, self.left_bot.x, PLAYER_SIZE.x);
			let ydist = dist(remaining_vel.y, self.left_bot.y, PLAYER_SIZE.y);

			assert!(xdist > 0);
			assert!(ydist > 0);

			if xdist > remaining_vel.x.abs() && ydist > remaining_vel.y.abs() { // if no more collisions can happen!
				self.left_bot += remaining_vel;
				break;
			} else if (xdist * self.velocity.y).abs() < (ydist * self.velocity.x).abs() { //    <->    xdist / self.velocity.x < ydist / self.velocity.y    <->    xtime < ytime
				assert!(false);

				let change_x = xdist * remaining_vel.x.signum();
				let change_y = xdist * remaining_vel.y / remaining_vel.x.abs();
				let change = Vec2i::new(change_x, change_y);

				remaining_vel -= change;
				self.left_bot += change;

				// TODO this should only happen upon collision!
				// remaining_vel.x = 0;
				// self.velocity.x = 0;
			} else {
				let change_x = ydist * remaining_vel.x / remaining_vel.y.abs();
				let change_y = ydist * remaining_vel.y.signum();
				let change = Vec2i::new(change_x, change_y);

				assert!(change != 0.into());

				remaining_vel -= change;
				self.left_bot += change;

				// remaining_vel.y = 0;
				// self.velocity.y = 0;
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

fn dist(velocity: i32, min: i32, size: i32) -> i32 {
	if velocity < 0 {
		(min % TILESIZE) + 1
	} else {
		let max = min + size - 1;
		(max / TILESIZE) + 1
	}
}
