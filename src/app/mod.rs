use sfml::graphics::{RenderWindow, Color, RenderTarget};
use sfml::window::{Style, VideoMode, Event, Key};

use crate::world::World;

pub struct App {
	window: RenderWindow,
	world: World,
}

impl App {
	pub fn new() -> App {
        let desktop = VideoMode::desktop_mode();
		App {
			window: RenderWindow::new(VideoMode::new(1280, 720, desktop.bits_per_pixel), "Elements 2", Style::CLOSE, &Default::default()),
			world: World::new(),
		}
	}

	pub fn run(&mut self) {
        while self.window.is_open() {
			while let Some(event) = self.window.poll_event() {
                match event {
                    Event::Closed | Event::KeyPressed { code: Key::Q, .. } => {
                        self.window.close();
                        return;
                    }
                    _ => {},
                }
			}

			// self.tick();
			// self.render();

			self.window.display();
			self.window.clear(Color::rgb(0, 0, 0));

			std::thread::sleep(std::time::Duration::from_millis(10));
		}
	}
}
