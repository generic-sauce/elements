use crate::prelude::*;

pub struct Menu {
	buttons: Vec<Button>,
}

pub struct Button {
	position: CanvasVec,
	size: CanvasVec,
}

pub struct MenuRunnable {
	pub menu: Menu,
}

impl Menu {
	pub fn main_menu() -> Menu {
		Menu {
			buttons: Vec::new(),
		}
	}
}

impl MenuRunnable {
	pub fn new() -> MenuRunnable {
		MenuRunnable {
			menu: Menu::main_menu(),
		}
	}
}

impl Runnable for MenuRunnable {
	fn tick(&mut self, app: &mut App) {
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
		context.draw_rect(&app.window, CanvasVec::new(0.1, 0.2), CanvasVec::new(0.1, 0.1), Color::rgb(0, 255, 0), Origin::LeftBottom);
	}
}