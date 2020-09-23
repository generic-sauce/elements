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
		println!("menu tick!");
	}

	fn draw(&mut self, app: &mut App, _timed_loop_info: &TimedLoopInfo) {
		app.window.display();
		app.window.clear(Color::BLACK);
	}
}