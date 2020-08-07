use crate::prelude::*;

pub struct Sensor {
	pub left_bot_offset: GameVec,
	pub size: GameVec,
}

impl Player {
	pub(in super) fn check_sensor(&self, s: &'static Sensor, t: &TileMap) -> bool {
		let lb = self.left_bot + s.left_bot_offset;
		let min = GameVec::new(lb.x, lb.y).to_tile();
		let max = GameVec::new(lb.x + s.size.x - 1, lb.y + s.size.y - 1).to_tile();

		// TODO write this using .any()?
		for x in min.x..=max.x {
			for y in min.y..=max.y {
				let p = TileVec::new(x, y);
				if t.get(p).is_solid() { return true; }
			}
		}
		false
	}
}
