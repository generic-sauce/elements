use crate::prelude::*;

pub struct OnlineMenuState {
	name_and_session_ids: Vec<(/* username */ String, /* session_id */ u32)>,
}

impl OnlineMenuState {
	pub fn new() -> OnlineMenuState {
		OnlineMenuState {
			name_and_session_ids: Vec::new(),
		}
	}
}
