use crate::prelude::*;

pub struct OnlineMenuState {
	name_and_session_ids: Vec<NameAndSessionId>,
}

impl OnlineMenuState {
	pub fn new() -> OnlineMenuState {
		OnlineMenuState {
			name_and_session_ids: Vec::new(),
		}
	}
}