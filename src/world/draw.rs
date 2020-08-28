use crate::prelude::*;

impl World {
	pub fn draw(&self, target: &impl RenderTarget, context: &mut DrawContext) {
		self.fluidmap.draw(target, context);
		for pl in &self.players {
			pl.draw(target, context);
		}
		self.tilemap.draw(target, context);
	}
}
