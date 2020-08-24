use crate::prelude::*;

impl TileMap {
	pub fn draw(&mut self, context: &mut DrawContext) {
		let shader = context.shader_state.get_shader(ShaderId::Tilemap);
		unsafe { shader.set_uniform_texture_raw("tilemap_tex", &self.texture); }

		let mut states = RenderStates::default();
		states.shader = Some(&shader.inner_shader);

		let mut rect = RectangleShape::default();
		rect.set_texture(&self.texture, true);
		rect.set_scale(Vector2f::new(1.0, -1.0));
		rect.set_size(Vector2f::new(context.aspect_ratio, -1.0));
		context.window.draw_rectangle_shape(&rect, states);
	}
}
