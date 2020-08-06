use crate::prelude::*;

impl FluidMap {
	pub fn draw(&self, context: &mut DrawContext) {
		let shader = &mut context.shader_state.get_shader(ShaderId::Fluid);

		let mut pixels = Vec::new();
		for (index, fluids) in self.grid.iter().enumerate() {
			if fluids.is_empty() {
				pixels.push(0 as u8);
				pixels.push(0 as u8);
				pixels.push(0 as u8);
				pixels.push(0 as u8);

			} else {
				let fluid = &fluids[0];
				let id = Vec2i::new(index as i32 % NUM_FLUID_CELLS.x, index as i32 / NUM_FLUID_CELLS.y);
				let lv = fluid.position / 255 - id;
				println!("{} {}", id, lv);
				pixels.push(lv.x as u8);
				pixels.push(lv.y as u8);
				pixels.push((fluid.owner * 255) as u8);
				pixels.push(255);
			}
		}

		let image = Image::create_from_pixels(NUM_FLUID_CELLS.x as u32, NUM_FLUID_CELLS.y as u32, &pixels).unwrap();
		let mut texture_sfbox: SfBox<Texture> = Texture::from_image(&image).unwrap();
		let x: *mut Texture = &mut *texture_sfbox;
		let texture: &'static mut Texture;
		unsafe { texture = &mut *x; }

		shader.set_uniform_float("elapsed_time", context.elapsed_time.as_seconds());
		shader.set_uniform_texture("fluid_tex", texture);
		shader.set_uniform_vec2("fluid_tex_size", NUM_FLUID_CELLS.to_f().into());

		let mut states = RenderStates::default();
		states.shader = Some(&shader);

		let size = context.window.size();
		let mut rect = RectangleShape::default();
		rect.set_texture(&texture, true);
		rect.set_size(Vector2f::new(size.x as f32, size.y as f32));
		context.window.draw_rectangle_shape(&rect, states);
	}
}
