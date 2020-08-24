use crate::prelude::*;

impl World {
	pub fn draw(&mut self, context: &mut DrawContext) {
		self.fluidmap.draw(context);
		for p in self.players.iter_mut() {
			p.draw(context);
		}
		self.tilemap.draw(context);
	}
}
