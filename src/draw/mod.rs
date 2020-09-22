mod player;
mod fluidmap;
mod tilemap;
mod hud;

use crate::prelude::*;

impl App {
	pub fn draw(&mut self, elapsed_time: Duration, fps: u32, load: f32) {
		// by putting display at the start, the GPU has the whole frame to do graphics until .display() comes again
		// this offsets the rendering by one frame though!
		self.window.display();
		self.window.clear(Color::BLACK);

		let aspect_ratio = 16.0 / 9.0;
		let (window_view, view, view_pixel_size) = self.get_views(aspect_ratio);

		// declare render target
		let mut game_texture_target = RenderTexture::new(view_pixel_size.x as u32, view_pixel_size.y as u32, false).unwrap();
		let mut game_noise_target = RenderTexture::new(view_pixel_size.x as u32, view_pixel_size.y as u32, false).unwrap();

		game_texture_target.set_view(&view);
		game_noise_target.set_view(&view);
		self.window.set_view(&window_view);

		let window_size = self.window.size();
		let window_size = WindowVec::new(window_size.x as f32, window_size.y as f32);
		/*
		let mut context = DrawContext {
			window_size,
			texture_state: &self.texture_state,
			shader_state: &mut self.shader_state,
			font_state: &self.font_state,
			animation_state: &self.animation_state,
			tilemap_size: TileVec::new(100, 100),  // TODO: ?
			elapsed_time,
			tilemap_texture: &self.tilemap_texture,
			aspect_ratio,
		};

		// draw debug info
		let text_size = 0.030;
		context.draw_text(&self.window, CanvasVec::new(0.0, 1.0 - text_size * 2.0), text_size,
						  &format!("elapsed time: {}", elapsed_time.as_secs()), Origin::LeftTop);
		context.draw_text(&self.window, CanvasVec::new(0.0, 1.0 - text_size * 3.0), text_size,
						  &format!("fps: {}", fps as u32), Origin::LeftTop);
		context.draw_text(&self.window, CanvasVec::new(0.0, 1.0 - text_size * 4.0), text_size,
						  &format!("load: {:.2}%", load * 100.0), Origin::LeftTop);
		 */
	}

	fn get_views(&self, aspect_ratio: f32) -> (SfBox<View>, SfBox<View>, WindowVec) {
		let window_size = self.window.size();
		let window_size = Vector2f::new(window_size.x as f32, window_size.y as f32);
		let window_aspect_ratio = window_size.x / window_size.y;
		let aspect_factor = window_aspect_ratio / aspect_ratio;

		let wider_factor = f32::max(0.0, aspect_factor - 1.0) / 2.0;
		let higher_factor = f32::max(0.0, 1.0 / aspect_factor - 1.0) / 2.0;

		let left = -wider_factor * aspect_ratio;
		let width = aspect_ratio * (1.0 + 2.0 * wider_factor);
		let top = -higher_factor;
		let height = 1.0 + 2.0 * higher_factor;
		let window_view = View::from_rect(&FloatRect::new(left, 1.0 - top, width, -height));

		let wider_factor = wider_factor * 2.0 + 1.0;
		let higher_factor = higher_factor * 2.0 + 1.0;

		let view = View::from_rect(&FloatRect::new(0.0, 1.0, aspect_ratio, -1.0));
		let view_pixel_size = WindowVec::new(
			window_size.x as f32 / wider_factor,
			window_size.y as f32 / higher_factor
		);

		(window_view, view, view_pixel_size)
	}
}
