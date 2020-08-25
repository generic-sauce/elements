pub mod timed_loop;
mod resource;

pub use resource::res;

use crate::prelude::*;

pub struct App {
	window: RenderWindow,
	world: World,
	texture_state: TextureState,
	shader_state: ShaderState,
	font_state: FontState,
	animation_state: AnimationState,
	inputs: [Box<dyn Input>; 2],
	kills: [u32; 2],
	gilrs: gilrs::Gilrs,
}

impl App {
	pub fn new() -> App {
		let context_settings = ContextSettings::default();
		let gilrs = gilrs::Gilrs::new().expect("Failed to create gilrs");
		App {
			window: RenderWindow::new(VideoMode::desktop_mode(), "Elements 2", Style::DEFAULT, &context_settings),
			world: World::new(),
			texture_state: TextureState::new(),
			shader_state: ShaderState::new(),
			font_state: FontState::new(),
			animation_state: AnimationState::new(),
			inputs: [Box::new(AdaptiveInput::new(0, &gilrs)), Box::new(AdaptiveInput::new(1, &gilrs))],
			kills: [0, 0],
			gilrs,
		}
	}

	fn check_restart(&mut self) {
		if let Some(p) = (0..2).find(|&p| self.world.players[p].health == 0) {
			self.kills[1-p] += 1;
			self.world = World::new();
		}
	}

	pub fn run(&mut self) {
		let timed_loop = TimedLoop::with_fps(60);
		let interval = timed_loop.interval;
		for (elapsed_time, delta_time, fps, perf) in timed_loop {
			while let Some(event) = self.window.poll_event() {
				match event {
					Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => {
						self.window.close();
						return;
					}
					_ => {},
				}
			}

			// process gilrs events
			while let Some(_) = self.gilrs.next_event() {}

			if delta_time > interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", delta_time.as_millis(), interval.as_millis());
			}

			self.tick();
			self.draw(elapsed_time, fps, perf);

			self.window.display();
			self.window.clear(Color::BLACK);

			if !self.window.is_open() {
				break;
			}

			self.check_restart();
		}
	}

	pub fn tick(&mut self) {
		self.world.tick(&mut self.inputs, &self.gilrs);
	}

	pub fn draw(&mut self, elapsed_time: Duration, fps: u32, perf: f32) {
		let aspect_ratio = 16.0 / 9.0;
		let (window_view, view, view_pixel_size) = self.get_views(aspect_ratio);

		// declare render target
		let mut game_texture_target = RenderTexture::new(view_pixel_size.x, view_pixel_size.y, false).unwrap();
		let mut game_noise_target = RenderTexture::new(view_pixel_size.x, view_pixel_size.y, false).unwrap();

		game_texture_target.set_view(&view);
		game_noise_target.set_view(&view);
		self.window.set_view(&window_view);

		let window_size = self.window.size();
		let window_size = Vec2u::new(window_size.x, window_size.y);
		let mut context = DrawContext::new(
			window_size,
			&self.texture_state,
			&mut self.shader_state,
			&self.font_state,
			&self.animation_state,
			self.world.tilemap.size,
			elapsed_time,
			aspect_ratio,
		);

		// draw game
		context.fill_canvas_with_color(&game_texture_target, Color::rgb(115, 128, 56));
		self.world.draw(&game_texture_target, &mut context);
		context.apply_noise(&game_noise_target, game_texture_target);
		context.fill_canvas_with_texture(&self.window, game_noise_target);

		// draw debug info
		let text_size = 0.030;
		context.draw_text(&self.window, CanvasVec::new(0.0, 1.0 - text_size * 0.0), text_size,
			&format!("{} / {}", self.kills[0], self.kills[1]), Origin::LeftTop);

		context.draw_text(&self.window, CanvasVec::new(0.0, 1.0 - text_size * 2.0), text_size,
			&format!("elapsed time: {}", elapsed_time.as_secs()), Origin::LeftTop);
		context.draw_text(&self.window, CanvasVec::new(0.0, 1.0 - text_size * 3.0), text_size,
			&format!("fps: {}", fps as u32), Origin::LeftTop);
		context.draw_text(&self.window, CanvasVec::new(0.0, 1.0 - text_size * 4.0), text_size,
			&format!("perf: {:.2}%", perf), Origin::LeftTop);

		let (fluid_count_0, fluid_count_1) = (
			self.world.fluidmap.iter()
				.filter(|x| x.owner == 0)
				.count(),
			self.world.fluidmap.iter()
				.filter(|x| x.owner == 1)
				.count()
		);
		context.draw_text(&self.window, CanvasVec::new(0.0, 1.0 - text_size * 5.0), text_size,
			&format!("fluid count: {} / {}", fluid_count_0, fluid_count_1), Origin::LeftTop);
	}

	fn get_views(&self, aspect_ratio: f32) -> (SfBox<View>, SfBox<View>, Vec2u) {
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
		let view_pixel_size = Vec2u::new(
			(window_size.x as f32 / wider_factor) as u32,
			(window_size.y as f32 / higher_factor) as u32
		);

		(window_view, view, view_pixel_size)
	}
}
