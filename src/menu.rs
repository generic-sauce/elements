use crate::prelude::*;

const BUTTON_TEXT_SIZE: f32 = 0.05;

pub struct Menu {
	buttons: Vec<Button>,
}

pub struct Button {
	position: CanvasVec,
	size: CanvasVec,
	text: &'static str,
	is_clicked: bool,
}

pub struct MenuRunnable {
	pub menu: Menu,
	pub cursor_position: CanvasVec,
	pub next_runnable_change: RunnableChange,
}

impl Menu {
	pub fn main_menu() -> Menu {
		Menu {
			buttons: vec!(
				Button::new(CanvasVec::new(0.5 * 16.0 / 9.0, 0.6), CanvasVec::new(0.15, 0.05), "Best of 9"),
				Button::new(CanvasVec::new(0.5 * 16.0 / 9.0, 0.3), CanvasVec::new(0.15, 0.05), "Quit"),
			),
		}
	}
}

impl MenuRunnable {
	pub fn new() -> MenuRunnable {
		reset_mouse_position();
		MenuRunnable {
			menu: Menu::main_menu(),
			cursor_position: CanvasVec::new(0.5 * 16.0 / 9.0, 0.5),
			next_runnable_change: RunnableChange::None,
		}
	}
}

impl Button {
	fn new(position: CanvasVec, size: CanvasVec, text: &'static str) -> Button {
		Button {
			position,
			size,
			text,
			is_clicked: false,
		}
	}

	fn is_colliding(&self, pos: CanvasVec) -> bool {
		pos.x >= self.position.x - self.size.x && pos.x <= self.position.x + self.size.x &&
		pos.y >= self.position.y - self.size.y && pos.y <= self.position.y + self.size.y
	}
}

impl Runnable for MenuRunnable {
	fn tick(&mut self, _app: &mut App) {
		let mouse_update = get_mouse_position_update();
		self.cursor_position += CanvasVec::new(mouse_update.x, -mouse_update.y) * 0.001;

		if sfml::window::mouse::Button::Left.is_pressed() {
			for button in &mut self.menu.buttons {
				if button.is_colliding(self.cursor_position) {
					button.is_clicked = true;
				} else {
					button.is_clicked = false;
				}
			}
		} else {
			for (index, button) in &mut self.menu.buttons.iter_mut().enumerate() {
				if button.is_clicked {
					if index == 0 {
						self.next_runnable_change = RunnableChange::Game(9);
					}
					if index == 1 {
						self.next_runnable_change = RunnableChange::Quit;
					}
					button.is_clicked = false;
				}
			}
		}
	}

	fn draw(&mut self, app: &mut App, timed_loop_info: &TimedLoopInfo) {
		app.window.display();
		app.window.clear(Color::BLACK);

		let aspect_ratio = 16.0 / 9.0;
		let (window_view, view, view_pixel_size) = get_views(app, aspect_ratio);

		let mut game_texture_target = RenderTexture::new(view_pixel_size.x as u32, view_pixel_size.y as u32, false).unwrap();
		game_texture_target.set_view(&view);
		app.window.set_view(&window_view);

		let window_size = app.window.size();
		let window_size = WindowVec::new(window_size.x as f32, window_size.y as f32);

		let context = DrawContext {
			window_size,
			texture_state: &app.texture_state,
			shader_state: &mut app.shader_state,
			font_state: &app.font_state,
			animation_state: &app.animation_state,
			elapsed_time: timed_loop_info.elapsed_time,
			aspect_ratio,
		};

		context.fill_canvas_with_color(&game_texture_target, Color::rgb(255, 0, 0));

		// draw buttons
		for button in &self.menu.buttons {
			let color = if button.is_clicked {
				Color::rgb(47, 110, 140)
			} else if button.is_colliding(self.cursor_position) {
				Color::rgb(32, 82, 120)
			} else {
				Color::rgb(21, 67, 109)
			};
			context.draw_rect(&app.window, button.position, button.size, color, Origin::Center);
			context.draw_text(&app.window, button.position - CanvasVec::new(button.text.len() as f32 * BUTTON_TEXT_SIZE / 5.5, 0.45 * BUTTON_TEXT_SIZE), BUTTON_TEXT_SIZE, &button.text, Origin::LeftBottom);
		}

		// draw cursor
		context.draw_circle(&app.window, self.cursor_position, 0.01, Color::BLACK);
		context.draw_circle(&app.window, self.cursor_position, 0.008, Color::WHITE);
	}

	fn get_runnable_change(&mut self) -> RunnableChange {
		self.next_runnable_change
	}
}