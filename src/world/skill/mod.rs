use crate::prelude::*;

impl World {
	pub(in super) fn handle_skills(&mut self, inputs: &mut [Box<dyn Input>; 2]) {
		for p in 0..2 {
			if inputs[p].attack1() { self.handle_throw(p); }
			if inputs[p].attack2() { self.handle_throw3(p); }
			if inputs[p].special1() { self.handle_ice(p); }
		}
	}

	fn handle_throw(&mut self, p: usize) {
		self.fluidmap.iter_mut_notranslate()
			.filter(|x| x.owner == p)
			.for_each(|f| f.state = FluidState::Free)
	}

	fn handle_throw3(&mut self, p: usize) {
		self.fluidmap.iter_mut_notranslate()
			.filter(|x| x.owner == p)
			.take(3)
			.for_each(|f| f.state = FluidState::Free)
	}

	fn handle_ice(&mut self, p: usize) {
		// TODO decrease fluids count
		let player = &self.players[p];
		let cursor_tile: TileVec = player.cursor_position().into();
		let tile = self.tilemap.get(cursor_tile);

		if tile != Tile::Void { return; }
		if (0..2).any(|i| self.players[i].collides_tile(cursor_tile)) {
			return;
		}

		self.tilemap.set(cursor_tile, Tile::Wall { owner: p, remaining_lifetime: WALL_LIFETIME });
	}
}
