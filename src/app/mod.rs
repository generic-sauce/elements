mod runnable;
pub use runnable::*;

use crate::prelude::*;

pub const DEFAULT_CURSOR_POSITION: CanvasVec = CanvasVec::new(0.5 * 16.0 / 9.0, 0.5);

pub const TICK_FPS: f64 = 60.0;

pub struct App<B: Backend> {
	pub input_backend: B::InputBackend,
	pub graphics_backend: B::GraphicsBackend,
	pub audio_backend: B::AudioBackend,
	pub cursor_position: CanvasVec,
	pub peripherals_state: PeripheralsState,
	pub menu: Menu<B>,
	pub timer: Timer,
	pub tick_counter: u32,
}

impl<B: Backend> App<B> {
	pub fn new(graphics_backend: B::GraphicsBackend, input_backend: B::InputBackend, menu: Menu<B>) -> App<B> {
		App {
			input_backend,
			graphics_backend,
			audio_backend: B::AudioBackend::new(),
			cursor_position: DEFAULT_CURSOR_POSITION,
			peripherals_state: PeripheralsState::new(),
			menu,
			timer: Timer::new(),
			tick_counter: 0,
		}
	}

	pub fn fetch_peripherals(&mut self) {
		for ev in self.input_backend.events() {
			self.peripherals_state.update(&ev);
		}
	}

	fn update_cursor(&mut self) {
		let mouse_update = self.peripherals_state.cursor_move;
		self.cursor_position += mouse_update.cast() * 0.001 * (1.0, -1.0);
		self.cursor_position.y = self.cursor_position.y.max(0.0).min(1.0);
		self.cursor_position.x = self.cursor_position.x.max(0.0).min(ASPECT_RATIO);
	}

	fn tick_fps(&self) -> f64 {
		1000.0 * self.tick_counter as f64 / self.timer.elapsed_ms()
	}

	pub fn tick_draw(&mut self, runnable: &mut Runnable<B>) {
		let repeat_opt = (0..10).find(|_| {
			self.fetch_peripherals();
			self.input_backend.tick();
			self.update_cursor();

			runnable.tick(self);
			self.tick_counter += 1;

			self.tick_fps() >= TICK_FPS
		});

		if repeat_opt.is_none() {
			println!("App::tick_draw experienced a framedrop.");
		}

		runnable.draw(self);

		// TODO: improve
		if let Runnable::Menu = runnable {
			self.tick_menu(runnable);
			self.draw_menu();
		}

		self.audio_backend.tick();
		self.peripherals_state.reset();
	}

	fn handle(&mut self, handler: &AppEventHandler) {
		if let Some(dmg) = (0..2).map(|p| handler.damages[p]).max() {
			if dmg > 0 {
				let volume = (dmg as f32 / 100.0).max(0.5).min(2.0);
				self.audio_backend.play_sound(SoundId::Whiz, volume);
			}
		}
	}
}

impl World {
	pub fn tick_within_app<B: Backend>(&mut self, app: &mut App<B>) {
		let mut handler = AppEventHandler::new();
		self.tick(&mut handler);
		app.handle(&handler);

		self.update_music_within_app(app);
	}

	fn update_music_within_app<B: Backend>(&mut self, app: &mut App<B>) {
		let mut critical_level = 0;
		for player in &self.players {
			if player.health < MAX_HEALTH / 2 {
				critical_level += 1;
			}
		}
		let sound_id = [SoundId::APart, SoundId::BPart, SoundId::DPart][critical_level];
		if app.audio_backend.current_music_id().map_or(true, |music_id| music_id != sound_id) {
			app.audio_backend.queue_music(sound_id);
		}
	}

	pub fn apply_update_within_app<B: Backend>(&mut self, update: WorldUpdate, app: &mut App<B>) {
		let mut handler = AppEventHandler::new();
		self.apply_update(update, &mut handler);
		app.handle(&handler);
	}
}