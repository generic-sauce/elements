use crate::prelude::*;

pub struct AppEventHandler {
	pub tilemap_changed: bool,
	pub damages: Vec<i32>,
	pub new_game_started: bool,
	pub game_ended: bool,
}

impl AppEventHandler {
	pub fn new(no_players: usize) -> AppEventHandler {
		AppEventHandler {
			tilemap_changed: false,
			damages: vec![0; no_players],
			new_game_started: false,
			game_ended: false,
		}
	}
}

impl EventHandler for AppEventHandler {
	fn tilemap_changed(&mut self) {
		self.tilemap_changed = true;
	}

	fn damage_inflicted(&mut self, damage: i32, player: usize) {
		self.damages[player] += damage;
	}

	fn new_game_started(&mut self) {
		self.new_game_started = true;
	}

	fn game_ended(&mut self) {
		self.game_ended = true;
	}
}
