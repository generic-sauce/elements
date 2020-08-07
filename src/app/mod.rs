pub mod timed_loop;

use crate::prelude::*;

pub struct App {
	window: RenderWindow,
	world: World,
	texture_state: TextureState,
	shader_state: ShaderState,
	font_state: FontState,
	animation_state: AnimationState,
	inputs: [Box<dyn Input>; 2],
	clock: Clock,
	smooth_fps: f32,
	smooth_perf: f32,
}

impl App {
	pub fn new() -> App {
		let context_settings = ContextSettings::default();
		App {
			window: RenderWindow::new(VideoMode::desktop_mode(), "Elements 2", Style::FULLSCREEN, &context_settings),
			world: World::new(),
			texture_state: TextureState::new(),
			shader_state: ShaderState::new(),
			font_state: FontState::new(),
			animation_state: AnimationState::new(),
			inputs: [Box::new(AdaptiveInput::new(0)), Box::new(AdaptiveInput::new(1))],
			clock: Clock::start(),
			smooth_fps: 60.0,
			smooth_perf: 1.0,
		}
	}

	pub fn run(&mut self) {
		let timed_loop = TimedLoop::with_fps(60);
		let target_interval = timed_loop.interval;
		for delta_time in timed_loop {
			while let Some(event) = self.window.poll_event() {
				match event {
					Event::Closed | Event::KeyPressed { code: Key::Q, .. } => {
						self.window.close();
						return;
					}
					_ => {},
				}
			}

			println!("{}", delta_time.as_millis());
			let delta_time_f = delta_time.as_millis() as f32 + 1.0;
			let fps = 1000.0 / delta_time_f;
			self.smooth_fps = self.smooth_fps * 0.95 + fps * 0.05;
			let perf = delta_time_f / target_interval.as_millis() as f32;
			self.smooth_perf = self.smooth_perf * 0.95 + perf * 0.05;
			if delta_time > target_interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", delta_time.as_millis(), target_interval.as_millis());
			}

			self.tick();
			self.draw();

			self.window.display();
			self.window.clear(Color::rgb(0, 0, 0));

			if !self.window.is_open() {
				break;
			}

		}
	}

	pub fn tick(&mut self) {
		self.world.tick(&mut self.inputs);
	}

	pub fn draw(&mut self) {
		let mut context = DrawContext::new(
			&mut self.window,
			&self.texture_state,
			&mut self.shader_state,
			&self.font_state,
			&self.animation_state,
			self.world.tilemap.size,
			self.clock.elapsed_time());

		// draw game
		self.world.draw(&mut context);

		// draw time
		let mut elapsed_time = String::from("Elapsed time: ");
		elapsed_time.push_str(&self.clock.elapsed_time().as_seconds().floor().to_string());
		context.draw_text(Vec2f::new(20.0, 20.0), 32 as u32, &elapsed_time);
		context.draw_text(Vec2f::new(20.0, 60.0), 32 as u32, &format!("fps: {}", self.smooth_fps as u32));
		context.draw_text(Vec2f::new(20.0, 100.0), 32 as u32, &format!("perf: {:.0}%", self.smooth_perf * 100.0));
	}
}
