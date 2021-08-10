use crate::prelude::*;

pub enum Runnable<B: Backend> {
	Menu, // TODO: extract all menues later
	OnlineMenu(OnlineMenu<B>),
	Local(Local<B>),
	Client(Client<B>),
	ServerConnector(ServerConnector<B>),
}

impl<B: Backend> Runnable<B> {
	pub fn build_menu(&self, storage_backend: &B::StorageBackend) -> Menu<B> {
		match self {
			Runnable::Menu|Runnable::OnlineMenu(_) => Menu::online_menu(storage_backend),
			Runnable::Client(_) => Menu::in_game_menu(Box::new(create_online_menu)),
			Runnable::Local(_) => Menu::in_game_menu(Box::new(create_local_menu)),
			Runnable::ServerConnector(_) => Menu::new(),
		}
	}

	pub fn tick(&mut self, app: &mut App<B>) {
		let mut packets = Vec::new();
		while let Some(p) = app.master_socket.recv() {
			packets.push(p);
		}
		match self {
			Runnable::Menu => {},
			Runnable::OnlineMenu(online_menu) => online_menu.tick(app, packets),
			Runnable::Local(local) => local.tick(app),
			Runnable::Client(client) => client.tick(app),
			Runnable::ServerConnector(server_connector) => server_connector.tick(app),
		}
	}

	pub fn draw(&mut self, app: &mut App<B>, draw: &mut Draw) {
		match self {
			Runnable::Menu => {},
			Runnable::OnlineMenu(_) => {},
			Runnable::Local(local) => local.draw(app, draw),
			Runnable::Client(client) => client.draw(app, draw),
			Runnable::ServerConnector(server_connector) => server_connector.draw(app, draw),
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