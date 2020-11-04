use crate::prelude::*;

pub const DEFAULT_CURSOR_POSITION: CanvasVec = CanvasVec::new(0.5 * 16.0 / 9.0, 0.5);

pub struct App<B: Backend> {
	pub input_backend: B::InputBackend,
	pub graphics_backend: B::GraphicsBackend,
	pub audio_backend: B::AudioBackend,
	pub cursor_position: CanvasVec,
	pub peripherals_state: PeripheralsState,
}

pub trait Runnable<B: Backend> {
	fn tick(&mut self, app: &mut App<B>);
	fn draw(&mut self, app: &mut App<B>, elapsed_time: Duration);
	fn get_runnable_change(&mut self) -> RunnableChange;
}

#[derive(Copy, Clone, PartialEq)]
pub enum MenuChoice {
	Main,
	ConnectServer,
}

#[derive(Clone, PartialEq)]
pub enum RunnableChange {
	None,
	Quit,
	Local(u32),
	Menu(MenuChoice),
	Client(String),
}

impl RunnableChange {
	pub fn from_world(world: &World) -> RunnableChange {
		if world.best_of_n == 0 {
			return RunnableChange::None;
		}
		if world.restart_state == RestartState::Game {
			for kill in &world.kills {
				if kill >= &(world.best_of_n / 2 + 1) {
					return RunnableChange::Menu(MenuChoice::Main);
				}
			}
		}
		RunnableChange::None
	}
}

impl<B: Backend> App<B> {
	pub fn new(graphics_backend: B::GraphicsBackend, input_backend: B::InputBackend) -> App<B> {
		App {
			input_backend,
			graphics_backend,
			audio_backend: B::AudioBackend::new(),
			cursor_position: DEFAULT_CURSOR_POSITION,
			peripherals_state: PeripheralsState::new(),
		}
	}

	pub fn fetch_peripherals(&mut self) {
		for ev in self.input_backend.events() {
			self.peripherals_state.update(&ev);
		}
	}

	pub fn run_local(&mut self, best_of_n: u32) {
		self.run_runnable(Local::new(best_of_n));
	}

	pub fn run_client(&mut self, ip: &str) {
		self.run_runnable(Client::new(ip));
	}

	pub fn run_menu_and_game(&mut self) {
		let mut runnable_change = RunnableChange::Menu(MenuChoice::Main);
		loop {
			runnable_change = match runnable_change {
				RunnableChange::None => panic!("should not receive RunnableChange::None from run_runnable"),
				RunnableChange::Local(best_of_n) => {
					self.run_local(best_of_n);
					RunnableChange::Menu(MenuChoice::Main)
				},
				RunnableChange::Quit => break,
				RunnableChange::Menu(choice) => self.run_runnable(MenuRunnable::new(choice)),
				RunnableChange::Client(ip) => {
					self.run_client(&ip);
					RunnableChange::Menu(MenuChoice::Main)
				},
			}
		}
	}

	fn run_runnable(&mut self, mut runnable: impl Runnable<B>) -> RunnableChange {
		let mut runnable_change = RunnableChange::None;

		for timed_loop_info in TimedLoop::with_fps(60) {
			self.fetch_peripherals();
			self.input_backend.tick();

			if timed_loop_info.delta_time > timed_loop_info.interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", timed_loop_info.delta_time.as_millis(), timed_loop_info.interval.as_millis());
			}

			runnable.tick(self);
			runnable.draw(self, timed_loop_info.elapsed_time);
			runnable_change = runnable.get_runnable_change();
			match runnable_change {
				RunnableChange::None => {},
				_ => break,
			}
			self.audio_backend.tick();
			self.peripherals_state.reset();
		};
		runnable_change
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