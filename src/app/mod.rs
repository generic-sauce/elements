use sfml::graphics::RenderWindow;
use sfml::window::{Style, VideoMode};

use crate::world::World;

pub struct App {
	window: RenderWindow,
	world: World,
}

impl App {
	pub fn new() -> App {
		App {
			window: RenderWindow::new(VideoMode::fullscreen_modes()[0], "Elements 2", Style::FULLSCREEN | Style::CLOSE, &Default::default()),
			world: World::new(),
		}
	}

	pub fn run(&mut self) {
		loop {
			println!("running!");
		}
	}
}
