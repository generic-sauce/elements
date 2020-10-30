use crate::prelude::*;

pub struct WebLocal;

impl WebLocal {
	pub fn new() -> Self {
		WebLocal
	}

	pub fn tick(&mut self, webapp_data: &mut WebappData) {
		for i in 0..2 {
			webapp_data.world.players[i].input.update_gamepad(&input_state(i));
		}
		webapp_data.world.tick(&mut ());
	}
}