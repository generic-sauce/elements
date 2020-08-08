use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct Animation {
	pub animation_id: AnimationId,
	pub index: usize,
}

impl Animation {
	pub fn new(animation_id: AnimationId) -> Animation {
		Animation {
			animation_id,
			index: 0,
		}
	}

	pub fn draw<T: IntoCanvasVec>(&mut self, position: T, radius: T, context: &DrawContext) {
		context.draw_animation(position, radius, *self);
		self.index = (self.index + 1) % (context.animation_state.get_frame_count(*self) * context.animation_state.get_interval(*self));
	}
}
