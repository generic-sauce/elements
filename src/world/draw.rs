use crate::prelude::*;

impl World {
	pub fn draw(&self, target: &impl RenderTarget, context: &mut DrawContext) {
		self.fluidmap.draw(target, context);
		for p in 0..2 {
			self.players[p].draw(p, target, context);
		}
		self.tilemap.draw(target, context);
	}
}
