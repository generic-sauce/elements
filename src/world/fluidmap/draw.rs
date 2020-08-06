use crate::prelude::*;

impl FluidMap {
	pub fn draw(&self, context: &mut DrawContext) {
		let shader = &mut context.shader_state.get_shader(ShaderId::Fluid);

		let size = (context.tilemap_size.x * context.tilemap_size.y) as usize;
		let mut fluid_grid = Vec::with_capacity(size);
		fluid_grid.resize(size, Vec2u8::new(0, 0));

		for fluids in self.grid.iter() {
			for fluid in fluids.iter() {
				let cell_id = fluid.position / 256;
				let local_position = Vec2u8::new((fluid.position.x % 256) as u8, (fluid.position.y % 256) as u8);
				let cell_index = cell_id.x + cell_id.y * context.tilemap_size.x as i32;
				fluid_grid[cell_index as usize] = local_position;
			}
		}

		let mut pixels = Vec::<u8>::new();
		for fluid in fluid_grid.iter() {
			if *fluid == Vec2u8::new(0, 0) {
				pixels.push(0);
				pixels.push(0);
				pixels.push(0);
				pixels.push(0);

			} else {
				pixels.push(fluid.x as u8);
				pixels.push(fluid.y as u8);
				pixels.push(0);
				pixels.push(255);
			}
		}

		let image = Image::create_from_pixels(context.tilemap_size.x as u32, context.tilemap_size.y as u32, &pixels).unwrap();
		let mut texture_sfbox: SfBox<Texture> = Texture::from_image(&image).unwrap();
		let x: *mut Texture = &mut *texture_sfbox;
		let texture: &'static mut Texture;
		unsafe { texture = &mut *x; }

		shader.set_uniform_float("elapsed_time", context.elapsed_time.as_seconds());
		shader.set_uniform_texture("fluid_tex", texture);
		shader.set_uniform_vec2("fluid_tex_size", context.tilemap_size.to_f().into());

		let mut states = RenderStates::default();
		states.shader = Some(&shader);

		let size = context.window.size();
		let mut rect = RectangleShape::default();
		rect.set_texture(&texture, true);
		rect.set_size(Vector2f::new(size.x as f32, size.y as f32));
		context.window.draw_rectangle_shape(&rect, states);
	}
}
