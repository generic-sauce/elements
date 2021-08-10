use crate::prelude::*;

pub struct OnlineMenu<B: Backend> {
	pub lobbies: Vec<ShortLobbyInfo>,
	_p: PhantomData<B>,
}

impl<B: Backend> OnlineMenu<B> {
	pub fn new() -> OnlineMenu<B> {

		OnlineMenu {
			lobbies: Vec::new(),
			_p: PhantomData,
		}
	}

	pub fn tick(&mut self, app: &mut App<B>, packets: Vec<MasterClientPacket>) {
		OnlineMenu::tick_username_field(app);

		for p in packets {
			match p {
				MasterClientPacket::LobbyListResponse(lobby_infos) => { self.lobbies = lobby_infos; }
				_ => eprintln!("WARN: Got invalid packet from master server"),
			}
		}
	}

	fn tick_username_field(app: &mut App<B>) {
		let e = app.menu_cache.edit_field.get("player_name").unwrap();
		if app.storage_backend.get("username").unwrap_or_else(String::new) != e.text {
			app.storage_backend.set("username", &e.text);
		}
	}
}
