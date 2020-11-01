mod player;
use player::*;

mod fluidmap;
mod tilemap;
mod hud;
// mod render;
// pub use render::*;

use crate::prelude::*;

impl ClientWorld {
	pub fn draw(&mut self, draw: &mut Draw) {
		draw.world(&self.world.tilemap, &self.world.fluidmap);
		draw_players(draw, &self.world);
		draw_cursors(draw, &self.world);
		draw_healthbars(draw, &self.world);
	}
}

/*
	pub fn draw(&mut self, app: &mut App, timed_loop_info: &TimedLoopInfo) {
		// by putting display at the start, the GPU has the whole frame to do graphics until .display() comes again
		// this offsets the rendering by one frame though!
		app.window.display();
		app.window.clear(Color::BLACK);

		let aspect_ratio = 16.0 / 9.0;
		let (window_view, view, view_pixel_size) = get_views(app, aspect_ratio);

		// // declare render target
		let mut game_texture_target = RenderTexture::new(view_pixel_size.x as u32, view_pixel_size.y as u32, false).unwrap();
		let mut game_noise_target = RenderTexture::new(view_pixel_size.x as u32, view_pixel_size.y as u32, false).unwrap();

		game_texture_target.set_view(&view);
		game_noise_target.set_view(&view);
		app.window.set_view(&window_view);

		let window_size = app.window.size();
		let window_size = WindowVec::new(window_size.x as f32, window_size.y as f32);
		let mut context = GameDrawContext {
			ctxt: DrawContext {
				window_size,
				texture_state: &app.texture_state,
				shader_state: &mut app.shader_state,
				font_state: &app.font_state,
				animation_state: &app.animation_state,
				elapsed_time: timed_loop_info.elapsed_time,
				aspect_ratio,
			},
			tilemap_size: self.world.tilemap.size,
			tilemap_texture: &self.tilemap_texture,
		};

		// let px_window = &app.px_window;
		// let px_pixels = &mut app.px_pixels;
		// self.renderer.update(&px_pixels.queue());
		// let render_result = px_pixels.render_with(|encoder, render_target, context| {
		// 	self.renderer.render(encoder, render_target);
		// });
    //
		// if render_result
		// 	.is_err()
		// {
		// 	dbg!("error drawing with pixels");
		// }

		// draw game
		context.ctxt.fill_canvas_with_color(&game_texture_target, Color::rgb(115, 128, 56));
		draw_world(&self.world, &game_texture_target, &mut context);
		context.ctxt.apply_noise(&game_noise_target, game_texture_target);
		context.ctxt.fill_canvas_with_texture(&app.window, game_noise_target);

		// draw debug info
		let text_size = 0.030;
		let best_of_str = if self.world.best_of_n == 0 {
			String::from("infinite game")
		} else {
			format!("best of {}", self.world.best_of_n)
		};
		context.draw_text(&app.window, CanvasVec::new(0.0, 1.0 - text_size * 0.0), text_size,
						  &format!("{} / {}   {}", self.world.kills[0], self.world.kills[1], best_of_str), Origin::LeftTop);
		context.draw_text(&app.window, CanvasVec::new(0.0, 1.0 - text_size * 2.0), text_size,
						  &format!("elapsed time: {}", timed_loop_info.elapsed_time.as_secs()), Origin::LeftTop);
		context.draw_text(&app.window, CanvasVec::new(0.0, 1.0 - text_size * 3.0), text_size,
						  &format!("fps: {}", timed_loop_info.fps as u32), Origin::LeftTop);
		context.draw_text(&app.window, CanvasVec::new(0.0, 1.0 - text_size * 4.0), text_size,
						  &format!("load: {:.2}%", timed_loop_info.load * 100.0), Origin::LeftTop);
		context.draw_text(&app.window, CanvasVec::new(0.0, 1.0 - text_size * 5.0), text_size,
						  &format!(
							  "num fluids: {} / {}",
							  self.world.fluidmap.iter().filter(|f| f.owner == 0).count(),
							  self.world.fluidmap.iter().filter(|f| f.owner == 1).count()
						  ), Origin::LeftTop);
	}

}

pub fn get_views(app: &App, aspect_ratio: f32) -> (SfBox<View>, SfBox<View>, WindowVec) {
	let window_size = app.window.size();
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

fn draw_world(w: &World, target: &impl RenderTarget, context: &mut GameDrawContext) {
	fluidmap::draw(&w.fluidmap, target, context);
	for pl in &w.players {
		player::draw(pl, target, context);
	}
	tilemap::draw(target, context);
	for pl in &w.players {
		hud::draw(pl, target, context);
	}
}
*/
