mod sound;

pub use sound::*;
use crate::prelude::*;

pub struct App {
	pub window: RenderWindow,
	pub px_window: pxp::Window,
	pub texture_state: TextureState,
	pub shader_state: ShaderState,
	pub font_state: FontState,
	pub animation_state: AnimationState,
	pub gilrs: gilrs::Gilrs,
	pub sound_manager: SoundManager,
}

// TODO: rename
pub trait Runnable {
	fn tick(&mut self, app: &mut App);
	fn draw(&mut self, app: &mut App, timed_loop_info: &TimedLoopInfo);
}

impl App {
	pub fn new(px_window: pxp::Window) -> App {
		let context_settings = ContextSettings::default();
		let mut window = RenderWindow::new(VideoMode::desktop_mode(), "Elements 2", Style::DEFAULT, &context_settings);
		window.set_mouse_cursor_visible(false);

		let gilrs = gilrs::Gilrs::new().expect("Failed to create gilrs");

		App {
			window,
			px_window,
			texture_state: TextureState::new(),
			shader_state: ShaderState::new(),
			font_state: FontState::new(),
			animation_state: AnimationState::new(),
			gilrs,
			sound_manager: SoundManager::new(),
		}
	}

	pub fn run_local(&mut self) {
		self.run_runnable(Local::new(&self.gilrs));
	}

	pub fn run_client(&mut self, ip: &str) {
		self.run_runnable(Client::new(ip, &self.gilrs));
	}

	fn run_runnable(&mut self, mut runnable: impl Runnable) {
		for timed_loop_info in TimedLoop::with_fps(60) {
			while let Some(event) = self.window.poll_event() {
				match event {
					Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => {
						self.window.close();
						std::process::exit(0);
					}
					_ => {},
				}
			}
			// process gilrs events
			while self.gilrs.next_event().is_some() {}

			if timed_loop_info.delta_time > timed_loop_info.interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", timed_loop_info.delta_time.as_millis(), timed_loop_info.interval.as_millis());
			}

			runnable.tick(self);
			runnable.draw(self, &timed_loop_info);
			self.sound_manager.tick();

			if !self.window.is_open() {
				std::process::exit(0);
			}
		}
	}
}
