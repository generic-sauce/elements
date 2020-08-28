use crate::prelude::*;

#[derive(Copy, Clone, Serialize, Deserialize)]
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

	pub fn tick(&mut self, animation_state: &AnimationState) {
		self.index = (self.index + 1) % (animation_state.get_frame_count(*self) * animation_state.get_interval(*self));
	}

	pub fn draw(&self, target: &impl RenderTarget, flip: Flip, position: impl IntoCanvasVec, radius: impl IntoCanvasVec, context: &DrawContext) {
		context.draw_animation(target, position, radius, *self, flip);
	}
}
