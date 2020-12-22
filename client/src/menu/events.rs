use crate::prelude::*;

pub type OnEvent<B> = Box<dyn OnEventImpl<B>>;

pub fn noop<B: Backend>(_app: &mut App<B>, _runnable: &mut Runnable<B>) {}

pub fn create_join_server_menu<B: Backend>(app: &mut App<B>, _runnable: &mut Runnable<B>) {
	app.menu = Menu::connect_server_menu();
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

pub fn create_quick_play_menu<B: Backend>(app: &mut App<B>, runnable: &mut Runnable<B>) {
	*runnable = Runnable::Menu;
	app.menu = Menu::quick_play_menu();
}

pub fn create_local_menu<B: Backend>(app: &mut App<B>, _runnable: &mut Runnable<B>) {
	app.menu = Menu::local_menu();
}

pub fn create_tutorial_menu<B: Backend>(app: &mut App<B>, _runnable: &mut Runnable<B>) {
	app.menu = Menu::tutorial_menu();
}
