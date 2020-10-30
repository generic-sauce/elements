mod sound;

pub use sound::*;
use crate::prelude::*;

pub const DEFAULT_CURSOR_POSITION: CanvasVec = CanvasVec::new(0.5 * 16.0 / 9.0, 0.5);

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
	pub input_receiver: Receiver<PeripheralsUpdate>,
	pub peripherals_state: PeripheralsState,
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
	pub fn new(graphics_sender: Sender<GraphicsWorld>, input_receiver: Receiver<PeripheralsUpdate>) -> App {
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
			input_receiver,
			peripherals_state: PeripheralsState::new(),
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

	fn fetch_peripherals_update(&mut self) {
		let receive = |app: &mut App| app.input_receiver.try_recv().map_err(|err| match err {
			TryRecvError::Disconnected => panic!("PeripheralsUpdate Sender disconnected!"),
			x => x,
		});
		while let Ok(peripherals_update) = receive(self) {
			self.peripherals_state.update(&peripherals_update);
		}
	}

	fn run_runnable(&mut self, mut runnable: impl Runnable) -> RunnableChange {
		let mut runnable_change = RunnableChange::None;

		for timed_loop_info in TimedLoop::with_fps(60) {
			// process gilrs events
			while self.gilrs.next_event().is_some() {}

			self.fetch_peripherals_update();

			if timed_loop_info.delta_time > timed_loop_info.interval {
				println!("Framedrop. Frame took {}ms instead of {}ms", timed_loop_info.delta_time.as_millis(), timed_loop_info.interval.as_millis());
			}

			runnable.tick(self);
			runnable.draw(self, &timed_loop_info);
			runnable_change = runnable.get_runnable_change();
			match runnable_change {
				RunnableChange::None => {},
				_ => { break; },
			}
			self.sound_manager.tick();
			self.peripherals_state.reset();
		};
		runnable_change
	}
}
