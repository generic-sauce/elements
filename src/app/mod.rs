use crate::prelude::*;

pub struct App {
	window: RenderWindow,
	world: World,
	texture_state: TextureState,
	shader_state: ShaderState,
	inputs: [Box<dyn Input>; 2],
}

impl App {
	pub fn new() -> App {
        let context_settings = ContextSettings::default();
		App {
			window: RenderWindow::new(VideoMode::desktop_mode(), "Elements 2", Style::FULLSCREEN, &context_settings),
			world: World::new(),
			texture_state: TextureState::new(),
			shader_state: ShaderState::new(),
			inputs: [Box::new(AdaptiveInput::new(0)), Box::new(AdaptiveInput::new(1))]
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

			self.tick();
			self.draw();

			self.window.display();
			self.window.clear(Color::rgb(0, 0, 0));

			std::thread::sleep(std::time::Duration::from_millis(10));
		}
	}

	pub fn tick(&mut self) {
		self.world.tick(&mut self.inputs);
	}

	pub fn draw(&mut self) {
        let mut context = Context::new(&mut self.window, &self.texture_state, &mut self.shader_state, self.world.tilemap.size);
		self.world.draw(&mut context);
        context.draw_text(Vec2f::new(20.0, 20.0), 32 as u32, "Draw some text (but load the font every frame LoL)");
	}
}
