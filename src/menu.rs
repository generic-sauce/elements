use crate::prelude::*;

pub struct Menu {
	buttons: Vec<Button>,
}

pub struct Button {
	position: CanvasVec,
}

impl Menu {
	pub fn main_menu() -> Menu {
		Menu {
			buttons: Vec::new(),
		}
	}
}