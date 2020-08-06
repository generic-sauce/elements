use crate::prelude::*;

impl TileMap {
	pub fn draw(&mut self, context: &mut DrawContext) {
		let shader = &mut context.shader_state.get_shader(ShaderId::Tilemap);
		let x: *mut Texture = &mut *self.texture;
		let texture: &'static mut Texture;
		unsafe { texture = &mut *x; }

		shader.set_uniform_texture("tilemap_tex", texture);

		let mut states = RenderStates::default();
		states.shader = Some(&shader);

		let size = context.window.size();
		let mut rect = RectangleShape::default();
		rect.set_texture(&texture, true);
		rect.set_size(Vector2f::new(size.x as f32, size.y as f32));
		context.window.draw_rectangle_shape(&rect, states);
	}
}
