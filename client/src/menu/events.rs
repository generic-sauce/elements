use crate::prelude::*;

pub type OnEvent<B> = Box<dyn OnEventImpl<B>>;

pub fn create_local<B: Backend>(best_of_n: u32) -> OnEvent<B> {
	Box::new(move |app, runnable| {
		app.menu = Menu::in_game_menu();
		*runnable = Runnable::Local(Local::new(best_of_n));
	})
}

pub fn create_client<B: Backend>() -> OnEvent<B> {
	Box::new(|app, runnable| {
		if let MenuKind::EditField( EditField { text, .. } ) = &app.menu.get_element_by_name("ip").unwrap().kind {
			let text = text.clone();
			app.menu = Menu::in_game_menu();
			*runnable = Runnable::Client(Client::new(&text, DEFAULT_GAME_SERVER_PORT));
		} else {
			panic!("Could not read ip from edit field!");
		}
	})
}

pub fn create_server_connector<B: Backend>(app: &mut App<B>, runnable: &mut Runnable<B>) {
	let player_name = app.menu.get_element_by_name("player_name").map(|ef| {
		match &ef.kind {
			MenuKind::EditField(edit_field) => &edit_field.text,
			_ => panic!("got player_name menu item that is not an edit field!")
		}
	}).expect("Could not find player name");

	*runnable = Runnable::ServerConnector(ServerConnector::new("generic-sauce.de", player_name));
	app.menu = Menu::server_connector_menu();
}

pub fn create_online_menu<B: Backend>(app: &mut App<B>, runnable: &mut Runnable<B>) {
	*runnable = Runnable::Menu;
	app.menu = Menu::online_menu();
}

pub fn create_local_menu<B: Backend>(app: &mut App<B>, runnable: &mut Runnable<B>) {
	*runnable = Runnable::Menu;
	app.menu = Menu::local_menu();
}

pub fn create_tutorial_menu<B: Backend>(app: &mut App<B>, runnable: &mut Runnable<B>) {
	*runnable = Runnable::Menu;
	app.menu = Menu::tutorial_menu();
}
