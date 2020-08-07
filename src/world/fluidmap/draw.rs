use crate::prelude::*;

impl FluidMap {
	pub fn draw(&self, context: &mut DrawContext) {
		let shader = &mut context.shader_state.get_shader(ShaderId::Fluid);

		let size = (context.tilemap_size.x * context.tilemap_size.y) as usize;
		let mut pixels = Vec::with_capacity(size);
		pixels.resize(size * 4, 0 as u8);

		for fluid in self.iter() {
			let cell_id = fluid.position / TILESIZE;
			let local_position = Vec2u8::new((fluid.position.x % TILESIZE) as u8, (fluid.position.y % TILESIZE) as u8);

			let cell_index = 4 * (cell_id.x + cell_id.y * context.tilemap_size.x as i32) as usize;
			pixels[cell_index+3] = 255;
			pixels[cell_index+2] = 0;
			pixels[cell_index+1] = local_position.y as u8;
			pixels[cell_index+0] = local_position.x as u8;
		}

		let image = Image::create_from_pixels(context.tilemap_size.x as u32, context.tilemap_size.y as u32, &pixels).unwrap();
		let mut texture_sfbox: SfBox<Texture> = Texture::from_image(&image).unwrap();
		let x: *mut Texture = &mut *texture_sfbox;
		let texture: &'static mut Texture;
		unsafe { texture = &mut *x; }

		shader.set_uniform_float("elapsed_time", context.elapsed_time.as_seconds());
		shader.set_uniform_texture("fluid_tex", texture);
		let v = Vector2f::new(context.tilemap_size.x as f32, context.tilemap_size.y as f32); // TODO make nicer
		shader.set_uniform_vec2("fluid_tex_size", v);

		let mut states = RenderStates::default();
		states.shader = Some(&shader);

		let size = context.window.size();
		let mut rect = RectangleShape::default();
		rect.set_texture(&texture, true);
		rect.set_size(Vector2f::new(size.x as f32, size.y as f32));
		context.window.draw_rectangle_shape(&rect, states);
	}
}
