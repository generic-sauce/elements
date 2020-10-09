mod sound;

pub use sound::*;
use crate::prelude::*;

pub struct App {
	pub window: RenderWindow,
	pub texture_state: TextureState,
	pub shader_state: ShaderState,
	pub font_state: FontState,
	pub animation_state: AnimationState,
	pub gilrs: gilrs::Gilrs,
	pub sound_manager: SoundManager,
	pub cursor_position: CanvasVec,
}

pub struct KeyPressedEvent {
	pub code: Key,
	pub shift: bool,
	pub ctrl: bool,
}

impl KeyPressedEvent {
	pub fn to_char(&self) -> Option<char> {
		let mut c = match self.code {
			Key::A => 'a',
			Key::B => 'b',
			Key::C => 'c',
			Key::D => 'd',
			Key::E => 'e',
			Key::F => 'f',
			Key::G => 'g',
			Key::H => 'h',
			Key::I => 'i',
			Key::J => 'j',
			Key::K => 'k',
			Key::L => 'l',
			Key::M => 'm',
			Key::N => 'n',
			Key::O => 'o',
			Key::P => 'p',
			Key::Q => 'q',
			Key::R => 'r',
			Key::S => 's',
			Key::T => 't',
			Key::U => 'u',
			Key::V => 'v',
			Key::W => 'w',
			Key::X => 'x',
			Key::Y => 'y',
			Key::Z => 'z',
			Key::Num0 => '0',
			Key::Num1 => '1',
			Key::Num2 => '2',
			Key::Num3 => '3',
			Key::Num4 => '4',
			Key::Num5 => '5',
			Key::Num6 => '6',
			Key::Num7 => '7',
			Key::Num8 => '8',
			Key::Num9 => '9',
			Key::Period => '.',
			Key::Dash => '-',
			Key::Space => ' ',
			_ => return None,
		};
		if self.shift {
			c = c.to_uppercase().next().unwrap();
			if c == '.' {
				c = ':';
			} else if c == '-' {
				c = '_';
			}
		}
		Some(c)
	}
}

pub trait Runnable {
	fn tick(&mut self, app: &mut App);
	fn draw(&mut self, app: &mut App, timed_loop_info: &TimedLoopInfo);
	fn apply_key(&mut self, ev: &KeyPressedEvent);
	fn get_runnable_change(&mut self) -> RunnableChange;
}

#[derive(Copy, Clone, PartialEq)]
pub enum MenuChoice {
	Main,
	ConnectServer,
}

#[derive(Copy, Clone, PartialEq)]
pub enum RunnableChange {
	None,
	Quit,
	Game(u32),
	Menu(MenuChoice),
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
	pub fn new() -> App {
		let context_settings = ContextSettings::default();
		let mut window = RenderWindow::new(VideoMode::desktop_mode(), "Elements 2", Style::DEFAULT, &context_settings);
		window.set_mouse_cursor_visible(false);

		let gilrs = gilrs::Gilrs::new().expect("Failed to create gilrs");

		App {
			window,
			texture_state: TextureState::new(),
			shader_state: ShaderState::new(),
			font_state: FontState::new(),
			animation_state: AnimationState::new(),
			gilrs,
			sound_manager: SoundManager::new(),
			cursor_position: DEFAULT_CURSOR_POSITION,
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
			match runnable_change {
				RunnableChange::None => panic!("should not receive RunnableChange::None from run_runnable"),
				RunnableChange::Game(best_of_n) => { self.run_local(best_of_n); runnable_change = RunnableChange::Menu(MenuChoice::Main) },
				RunnableChange::Quit => break,
				RunnableChange::Menu(choice) => runnable_change = self.run_runnable(MenuRunnable::new(choice)),
			}
		}
	}

	fn run_runnable(&mut self, mut runnable: impl Runnable) -> RunnableChange {
		let mut runnable_change = RunnableChange::None;

		for timed_loop_info in TimedLoop::with_fps(60) {
			while let Some(event) = self.window.poll_event() {
				match event {
					Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => {
						self.window.close();
						std::process::exit(0);
					}
					Event::KeyPressed { code, shift, ctrl, .. } => { runnable.apply_key(&KeyPressedEvent { code, shift, ctrl })},
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
			runnable_change = runnable.get_runnable_change();
			match runnable_change {
				RunnableChange::Quit => { self.window.close(); break; },
				RunnableChange::Game(_) => { break; },
				RunnableChange::None => {},
				RunnableChange::Menu(_) => { break; },
			}
			self.sound_manager.tick();

			if !self.window.is_open() {
				std::process::exit(0);
			}
		};
		runnable_change
	}
}
