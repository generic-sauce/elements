mod sound;

pub use sound::*;
use crate::prelude::*;

pub const DEFAULT_CURSOR_POSITION: CanvasVec = CanvasVec::new(0.5 * 16.0 / 9.0, 0.5);

pub struct App<B: Backend> {
	pub input_backend: B::InputBackend,
	pub graphics_backend: B::GraphicsBackend,
	pub sound_manager: SoundManager,
	pub cursor_position: CanvasVec,
	pub peripherals_state: PeripheralsState,
}

pub trait Runnable<B: Backend> {
	fn tick(&mut self, app: &mut App<B>);
	fn draw(&mut self, app: &mut App<B>, timed_loop_info: &TimedLoopInfo);
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
			sound_manager: SoundManager::new(),
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
			runnable.draw(self, &timed_loop_info);
			runnable_change = runnable.get_runnable_change();
			match runnable_change {
				RunnableChange::None => {},
				_ => break,
			}
			self.sound_manager.tick();
			self.peripherals_state.reset();
		};
		runnable_change
	}
}
