use crate::prelude::*;

pub enum Runnable<B: Backend> {
	Menu,
	Local(Local<B>),
	Client(Client<B>),
	ServerConnector(ServerConnector<B>),
}

impl<B: Backend> Runnable<B> {
	pub fn build_menu(&self) -> Menu<B> {
		match self {
			Runnable::Menu => Menu::online_menu(),
			_ => Menu::new(),
		}
	}

	pub fn tick(&mut self, app: &mut App<B>) {
		match self {
			Runnable::Menu => {},
			Runnable::Local(local) => local.tick(app),
			Runnable::Client(client) => client.tick(app),
			Runnable::ServerConnector(server_connector) => server_connector.tick(app),
		}
	}

	pub fn draw(&mut self, app: &mut App<B>) {
		match self {
			Runnable::Menu => {},
			Runnable::Local(local) => local.draw(app),
			Runnable::Client(client) => client.draw(app),
			Runnable::ServerConnector(server_connector) => server_connector.draw(app),
		}
	}

	pub fn get_world(&self) -> Option<&World> {
		match self {
			Runnable::Local(Local { mode: LocalMode::InGame(world) }) => Some(world),
			Runnable::Client( Client { mode: ClientMode::InGame { world, .. }, .. }) => Some(world),
			_ => None,
		}
	}
}