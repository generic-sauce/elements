mod sound;

pub use sound::*;
use crate::prelude::*;

pub struct App {
	// pub winit_window: Box<winit::Window>,
	// pub wgpu_instance: wgpu::Instance,
	pub texture_state: TextureState,
	pub shader_state: ShaderState,
	pub font_state: FontState,
	pub animation_state: AnimationState,
	pub gilrs: gilrs::Gilrs,
	pub sound_manager: SoundManager,
	pub cursor_position: CanvasVec,
	pub graphics_sender: Sender<GraphicsWorld>,
	pub input_receiver: Receiver<KeyboardUpdate>,
}

pub trait Runnable {
	fn tick(&mut self, app: &mut App);
	fn draw(&mut self, app: &mut App, timed_loop_info: &TimedLoopInfo);
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

impl App {
	pub fn new(graphics_sender: Sender<GraphicsWorld>, input_receiver: Receiver<KeyboardUpdate>) -> App {
		let gilrs = gilrs::Gilrs::new().expect("Failed to create gilrs");

		App {
			texture_state: TextureState::new(),
			shader_state: ShaderState::new(),
			font_state: FontState::new(),
			animation_state: AnimationState::new(),
			gilrs,
			sound_manager: SoundManager::new(),
			cursor_position: DEFAULT_CURSOR_POSITION,
			graphics_sender,
			input_receiver
		}
	}

	pub fn run_local(&mut self, best_of_n: u32) {
		self.run_runnable(Local::new(&self.gilrs, best_of_n));
	}

	pub fn run_client(&mut self, ip: &str) {
		self.run_runnable(Client::new(ip, &self.gilrs));
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

	fn run_runnable(&mut self, mut runnable: impl Runnable) -> RunnableChange {
		let mut runnable_change = RunnableChange::None;

		for timed_loop_info in TimedLoop::with_fps(60) {
			// process gilrs events
			while self.gilrs.next_event().is_some() {}

			if timed_loop_info.delta_time > timed_loop_info.interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", timed_loop_info.delta_time.as_millis(), timed_loop_info.interval.as_millis());
			}

			runnable.tick(self);
			runnable.draw(self, &timed_loop_info);
			runnable_change = runnable.get_runnable_change();
			match runnable_change {
				RunnableChange::Quit => { break; },
				RunnableChange::Local(_) => { break; },
				RunnableChange::None => {},
				RunnableChange::Menu(_) => { break; },
				RunnableChange::Client(_) => { break; },
			}
			self.sound_manager.tick();
		};
		runnable_change
	}
}
