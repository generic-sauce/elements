use crate::prelude::*;

impl TileMap {
	pub fn draw(&mut self, target: &impl RenderTarget, context: &mut DrawContext) {
		let shader = context.shader_state.get_shader(ShaderId::Tilemap);
		let v = Vector2f::new(context.tilemap_size.x as f32, context.tilemap_size.y as f32); // TODO make nicer
		shader.set_uniform_vec2("tilemap_tex_size", v);
		unsafe { shader.set_uniform_texture_raw("tilemap_tex", &self.texture); }

		let mut states = RenderStates::default();
		states.shader = Some(&shader.inner_shader);

		let mut rect = RectangleShape::default();
		rect.set_texture(context.texture_state.get_texture(TextureId::Any), true);
		rect.set_scale(Vector2f::new(1.0, -1.0));
		rect.set_size(Vector2f::new(context.aspect_ratio, -1.0));
		target.draw_rectangle_shape(&rect, states);
	}
}
