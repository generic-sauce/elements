use crate::prelude::*;

pub struct TickInfo {
	pub player_damage: [i32, 2];
}

impl TickInfo {
	pub fn new() -> TickInfo {
		TickInfo {
			player_damage: [0, 0],
		}
	}

	pub fn record_player_damage(&mut self, player_index: usize, damage: i32) {
		self.player_damage[player_index] += damage;
	}

	// use a temporary TickInfo to update a non temporary one
	pub fn tick(tick_info: TickInfo) {
		
	}
}
