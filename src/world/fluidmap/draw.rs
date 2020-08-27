use crate::prelude::*;

impl FluidMap {
	pub fn draw(&self, target: &impl RenderTarget, context: &mut DrawContext) {
		let size = (context.tilemap_size.x * context.tilemap_size.y) as usize;
		let mut pixels = Vec::with_capacity(size);
		pixels.resize(size * 4, 0 as u8);

		for fluid in self.iter() {
			let cell_id = fluid.position / TILESIZE;
			let local_position = Vec2u8::new((fluid.position.x % TILESIZE) as u8, (fluid.position.y % TILESIZE) as u8);

			let cell_index = 4 * (cell_id.x + cell_id.y * context.tilemap_size.x as i32) as usize;
			pixels[cell_index+3] = 255;
			pixels[cell_index+2] = (fluid.owner * 255) as u8;
			pixels[cell_index+1] = local_position.y as u8;
			pixels[cell_index+0] = local_position.x as u8;
		}

		let shader = &mut context.shader_state.get_shader(ShaderId::Fluid);

		let image = Image::create_from_pixels(context.tilemap_size.x as u32, context.tilemap_size.y as u32, &pixels).unwrap();
		let container = TextureContainer::Boxed(Texture::from_image(&image).unwrap());

		shader.set_uniform_float("elapsed_time", context.elapsed_time.as_secs_f32());
		shader.set_uniform_texture("fluid_tex", container);
		let v = Vector2f::new(context.tilemap_size.x as f32, context.tilemap_size.y as f32); // TODO make nicer
		shader.set_uniform_vec2("fluid_tex_size", v);

		let mut states = RenderStates::default();
		states.shader = Some(&shader.inner_shader);

		let mut rect = RectangleShape::default();
		rect.set_texture(context.texture_state.get_texture(TextureId::Any), true);
		rect.set_size(Vector2f::new(context.aspect_ratio, 1.0));
		target.draw_rectangle_shape(&rect, states);

		#[cfg(debug_assertions)]
		for fluid in self.iter() {
			context.draw_circle(target, fluid.position, TILESIZE / 3, Color::GREEN);
			context.draw_circle(target, fluid.reference_position, TILESIZE / 5, Color::RED);
		}
	}
}
