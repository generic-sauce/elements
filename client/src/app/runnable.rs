use crate::prelude::*;

pub enum Runnable<B: Backend> { // the model (w.r.t. MVC) of our app
	OnlineMenu(OnlineMenu<B>),
	LobbyMenu(LobbyMenu<B>),
	LocalMenu,
	TutorialMenu,
	Local(Local<B>), // In-game in a local game
	Client(Client<B>), // In-game in a network game
}

impl<B: Backend> Runnable<B> {
	pub fn build_menu(&mut self, app: &mut App<B>) -> (Menu<B>, Option<OnEvent<B>>) {
		let mut menu = match self {
			Runnable::OnlineMenu(online_menu) => online_menu.build_menu(&app.menu_cache),
			Runnable::LobbyMenu(l) => l.build_menu(&app.menu_cache),
			Runnable::LocalMenu => Menu::local_menu(),
			Runnable::TutorialMenu => Menu::tutorial_menu(),
			Runnable::Client(_) => Menu::in_game_menu(Box::new(create_online_menu)),
			Runnable::Local(_) => Menu::in_game_menu(Box::new(create_local_menu)),
		};

		let opt_on_click = menu.calc_element_properties(app, self);

		menu.init_cache(&mut app.menu_cache);
		(menu, opt_on_click)
	}

	pub fn tick(&mut self, app: &mut App<B>) {
		let mut packets = Vec::new();
		if let Some(s) = &mut app.master_socket {
			loop {
				match s.recv() {
					Ok(Some(x)) => packets.push(x),
					Ok(None) => break,
					Err(x) => {
						eprintln!("Runnable::tick() packet receiving failed due to \"{}\"", x);
						break;
					}

				}
			}
		}
		match self {
			Runnable::OnlineMenu(online_menu) => {
				if let Some(long_lobby_info) = online_menu.tick(app, packets) {
					*self = Runnable::LobbyMenu(LobbyMenu::from_lobby_info(long_lobby_info));
				}
			},
			Runnable::LocalMenu => {},
			Runnable::LobbyMenu(x) => {
				if let Some((domain, port)) = x.tick(packets) {
					match Client::new(&domain, port) {
						Ok(c) => *self = Runnable::Client(c),
						Err(x) => eprintln!("Runnable::tick(): can't open client due to \"{}\"", x),
					}
				}
			},
			Runnable::TutorialMenu => {},
			Runnable::Local(local) => local.tick(app),
			Runnable::Client(client) => client.tick(app),
		}
	}

	pub fn draw(&mut self, app: &mut App<B>, draw: &mut Draw) {
		match self {
			Runnable::OnlineMenu(_) => {},
			Runnable::LocalMenu => {},
			Runnable::LobbyMenu(_) => {},
			Runnable::TutorialMenu => {},
			Runnable::Local(local) => local.draw(app, draw),
			Runnable::Client(client) => client.draw(app, draw),
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