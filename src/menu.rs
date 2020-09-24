use crate::prelude::*;

const BUTTON_TEXT_SIZE: f32 = 0.05;

pub struct Menu {
	buttons: Vec<Button>,
}

pub struct Button {
	position: CanvasVec,
	size: CanvasVec,
	text: &'static str,
}

pub struct MenuRunnable {
	pub menu: Menu,
	pub cursor_position: CanvasVec,
}

impl Menu {
	pub fn main_menu() -> Menu {
		Menu {
			buttons: vec!(
				Button { position: CanvasVec::new(0.5 * 16.0 / 9.0, 0.6), size: CanvasVec::new(0.15, 0.05), text: "Best of five" },
				Button { position: CanvasVec::new(0.5 * 16.0 / 9.0, 0.3), size: CanvasVec::new(0.15, 0.05), text: "Quit" },
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
		}
	}
}

impl Runnable for MenuRunnable {
	fn tick(&mut self, app: &mut App) {
		let mouse_update = get_mouse_position_update();
		self.cursor_position += CanvasVec::new(mouse_update.x, -mouse_update.y) * 0.001;
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
			context.draw_rect(&app.window, button.position, button.size, Color::rgb(0, 255, 0), Origin::Center);
			context.draw_text(&app.window, button.position - CanvasVec::new(button.text.len() as f32 * BUTTON_TEXT_SIZE / 5.5, 0.45 * BUTTON_TEXT_SIZE), BUTTON_TEXT_SIZE, &button.text, Origin::LeftBottom);
		}

		// draw cursor
		context.draw_circle(&app.window, self.cursor_position, 0.01, Color::BLACK);
		context.draw_circle(&app.window, self.cursor_position, 0.008, Color::WHITE);
	}
}