use crate::prelude::*;

impl World {
	pub fn draw(&mut self, context: &mut DrawContext) {
		self.tilemap.draw(context);
		self.fluidmap.draw(context);
		for (i, p) in self.players.iter_mut().enumerate() {
			p.draw(i, context);
		}
	}
}
