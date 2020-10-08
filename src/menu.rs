use crate::prelude::*;

const BUTTON_TEXT_SIZE: f32 = 0.05;
pub const DEFAULT_CURSOR_POSITION: CanvasVec = CanvasVec::new(0.5 * 16.0 / 9.0, 0.5);
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;

pub struct Menu {
	pub buttons: Vec<Button>,
}

pub struct Button {
	position: CanvasVec,
	size: CanvasVec,
	text: &'static str,
	is_clicked: bool,
	runnable_change: RunnableChange,
}

pub struct MenuRunnable {
	pub menu: Menu,
	pub next_runnable_change: RunnableChange,
}

impl Menu {
	pub fn main_menu() -> Menu {
		Menu {
			buttons: vec!(
				Button::new(CanvasVec::new(0.3 * ASPECT_RATIO, 0.6), CanvasVec::new(0.15, 0.05), "Best of 9", RunnableChange::Game(9)),
				Button::new(CanvasVec::new(0.3 * ASPECT_RATIO, 0.4), CanvasVec::new(0.15, 0.05), "Best of 5", RunnableChange::Game(5)),
				Button::new(CanvasVec::new(0.7 * ASPECT_RATIO, 0.6), CanvasVec::new(0.15, 0.05), "Infinite Game", RunnableChange::Game(0)),
				Button::new(CanvasVec::new(0.7 * ASPECT_RATIO, 0.4), CanvasVec::new(0.15, 0.05), "Connect to Server", RunnableChange::Menu(MenuChoice::ConnectServer)),
				Button::new(CanvasVec::new(0.85 * ASPECT_RATIO, 0.15), CanvasVec::new(0.15, 0.05), "Quit", RunnableChange::Quit),
			),
		}
	}

	pub fn connect_server_menu() -> Menu {
		Menu {
			buttons: vec!(
				Button::new(CanvasVec::new(0.5 * 16.0 / 9.0, 0.6), CanvasVec::new(0.15, 0.05), "Connect", RunnableChange::Game(5)),
				Button::new(CanvasVec::new(0.5 * 16.0 / 9.0, 0.4), CanvasVec::new(0.15, 0.05), "Back", RunnableChange::Menu(MenuChoice::Main)),
				Button::new(CanvasVec::new(0.85 * 16.0 / 9.0, 0.15), CanvasVec::new(0.15, 0.05), "Quit", RunnableChange::Quit),
			)
		}
	}

	pub fn get_clicked_button(&mut self) -> Option<&mut Button> {
		self.buttons.iter_mut().find(|b| b.is_clicked)
	}
}

impl MenuRunnable {
	pub fn new(menu_choice: MenuChoice) -> MenuRunnable {
		reset_mouse_position();
		let menu = match menu_choice {
			MenuChoice::Main => Menu::main_menu(),
			MenuChoice::ConnectServer => Menu::connect_server_menu(),
		};
		MenuRunnable {
			menu,
			next_runnable_change: RunnableChange::None,
		}
	}
}

impl Button {
	fn new(position: CanvasVec, size: CanvasVec, text: &'static str, runnable_change: RunnableChange) -> Button {
		Button {
			position,
			size,
			text,
			is_clicked: false,
			runnable_change
		}
	}

	fn is_colliding(&self, pos: CanvasVec) -> bool {
		pos.x >= self.position.x - self.size.x && pos.x <= self.position.x + self.size.x &&
		pos.y >= self.position.y - self.size.y && pos.y <= self.position.y + self.size.y
	}
}

impl Runnable for MenuRunnable {
	fn tick(&mut self, app: &mut App) {
		let mouse_update = get_mouse_position_update();
		app.cursor_position += CanvasVec::new(mouse_update.x, -mouse_update.y) * 0.001;
		app.cursor_position.y = app.cursor_position.y.max(0.0).min(1.0);
		app.cursor_position.x = app.cursor_position.x.max(0.0).min(ASPECT_RATIO);

		if sfml::window::mouse::Button::Left.is_pressed() {
			for button in &mut self.menu.buttons {
				if button.is_colliding(app.cursor_position) {
					button.is_clicked = true;
				} else {
					button.is_clicked = false;
				}
			}
		} else {
			if let Some(button) = self.menu.get_clicked_button() {
				button.is_clicked = false;
				self.next_runnable_change = button.runnable_change;
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
			} else if button.is_colliding(app.cursor_position) {
				Color::rgb(32, 82, 120)
			} else {
				Color::rgb(21, 67, 109)
			};
			context.draw_rect(&app.window, button.position, button.size, color, Origin::Center);
			context.draw_text(&app.window, button.position - CanvasVec::new(button.text.len() as f32 * BUTTON_TEXT_SIZE / 5.5, 0.45 * BUTTON_TEXT_SIZE), BUTTON_TEXT_SIZE, &button.text, Origin::LeftBottom);
		}

		// draw cursor
		context.draw_circle(&app.window, app.cursor_position, 0.01, Color::BLACK);
		context.draw_circle(&app.window, app.cursor_position, 0.008, Color::WHITE);
	}

	fn get_runnable_change(&mut self) -> RunnableChange {
		self.next_runnable_change
	}
}