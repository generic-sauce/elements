use crate::prelude::*;

pub enum Runnable<B: Backend> { // the model (w.r.t. MVC) of our app
	OnlineMenu(OnlineMenu<B>),
	// LobbyMenu, // TODO add later
	LocalMenu,
	TutorialMenu,
	Local(Local<B>), // In-game in a local game
	Client(Client<B>), // In-game in a network game
	ServerConnector(ServerConnector<B>), // TODO obsolete
}

impl<B: Backend> Runnable<B> {
	pub fn build_menu(&mut self, app: &mut App<B>) -> (Menu<B>, Option<OnEvent<B>>) {
		let mut menu = match self {
			Runnable::OnlineMenu(_) => Menu::online_menu(),
			Runnable::LocalMenu => Menu::local_menu(),
			Runnable::TutorialMenu => Menu::tutorial_menu(),
			Runnable::Client(_) => Menu::in_game_menu(Box::new(create_online_menu)),
			Runnable::Local(_) => Menu::in_game_menu(Box::new(create_local_menu)),
			Runnable::ServerConnector(_) => Menu::new(),
		};

		let opt_on_click = menu.calc_element_properties(app, self);

		menu.init_cache(&mut app.menu_cache);
		(menu, opt_on_click)
	}

	pub fn tick(&mut self, app: &mut App<B>) {
		let mut packets = Vec::new();
		while let Some(p) = app.master_socket.recv() {
			packets.push(p);
		}
		match self {
			Runnable::OnlineMenu(online_menu) => online_menu.tick(app, packets),
			Runnable::LocalMenu => {},
			Runnable::TutorialMenu => {},
			Runnable::Local(local) => local.tick(app),
			Runnable::Client(client) => client.tick(app),
			Runnable::ServerConnector(server_connector) => server_connector.tick(app),
		}
	}

	pub fn draw(&mut self, app: &mut App<B>, draw: &mut Draw) {
		match self {
			Runnable::OnlineMenu(_) => {},
			Runnable::LocalMenu => {},
			Runnable::TutorialMenu => {},
			Runnable::Local(local) => local.draw(app, draw),
			Runnable::Client(client) => client.draw(app, draw),
			Runnable::ServerConnector(server_connector) => server_connector.draw(app, draw),
		}
	}

	pub fn get_world(&self) -> Option<&World> {
		match self {
			Runnable::Local(Local { mode: LocalMode::InGame(world), .. }) => Some(world),
			Runnable::Client( Client { mode: ClientMode::InGame { world, .. }, .. }) => Some(world),
			_ => None,
		}
	}

	pub fn is_active(&self) -> bool { // whether menu is active TODO rename
		match self {
			Runnable::Local(l) => l.active,
			Runnable::Client(c) => c.active,
			_ => true,
		}
	}

	pub fn toggle_active(&mut self) {
		match self {
			Runnable::Local(Local { active, .. }) => *active = !*active,
			Runnable::Client(Client { active, .. }) => *active = !*active,
			_ => {},
		}
	}
}