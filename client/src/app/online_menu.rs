use crate::prelude::*;

pub struct OnlineMenu<B: Backend> {
	pub name_and_session_ids: Vec<(/* username */ String, /* session_id */ u32)>,
	_p: PhantomData<B>,
}

impl<B: Backend> OnlineMenu<B> {
	pub fn new() -> OnlineMenu<B> {
		OnlineMenu {
			name_and_session_ids: Vec::new(),
			_p: PhantomData,
		}
	}

	pub fn tick(&self, app: &mut App<B>, packets: Vec<MasterClientPacket>) {
		OnlineMenu::tick_username_field(app);
	}

	fn tick_username_field(app: &mut App<B>) {
		let player_name_edit_field = app.menu.get_element_by_name("player_name").unwrap();
		match &player_name_edit_field.kind {
			MenuKind::EditField(edit_field) => {
				if app.storage_backend.get("username").unwrap_or_else(String::new) != edit_field.text {
					app.storage_backend.set("username", &edit_field.text);
				}
			},
			_ => panic!("player_name menu element should be edit field")
		}
	}
}
