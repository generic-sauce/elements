use crate::prelude::*;

impl World {
	pub fn draw(&mut self, target: &impl RenderTarget, context: &mut DrawContext) {
		self.fluidmap.draw(target, context);
		for p in self.players.iter_mut() {
			p.draw(target, context);
		}
		self.tilemap.draw(target, context);
	}
}
