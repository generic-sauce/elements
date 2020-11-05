use crate::prelude::*;

pub enum Runnable<B: Backend> {
	Menu,
	Local(Local<B>),
	Client(Client<B>),
}

impl<B: Backend> Runnable<B> {
	pub fn build_menu(&self) -> Menu<B> {
		match self {
			Runnable::Menu => Menu::main_menu(),
			_ => Menu::new(),
		}
	}

	pub fn tick(&mut self, app: &mut App<B>) {
		match self {
			Runnable::Menu => {},
			Runnable::Local(local) => local.tick(app),
			Runnable::Client(client) => client.tick(app),
		}
	}

	pub fn draw(&mut self, app: &mut App<B>) {
		match self {
			Runnable::Menu => {},
			Runnable::Local(local) => local.draw(app),
			Runnable::Client(client) => client.draw(app),
		}
	}
}