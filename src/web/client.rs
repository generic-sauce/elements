use crate::prelude::*;

#[derive(Serialize, Deserialize)]
enum Cmd {
	SendMsg { msg: Vec<u8> },
	Go { world_ptr: usize },
}

pub enum WebClientState {
	WaitingForGo,
	InGame {
		world: World,
		player_id: usize,
	},
}

#[wasm_bindgen]
pub fn new_webclient() -> *mut WebClientState {
	Box::leak(Box::new(WebClientState::WaitingForGo))
}

#[wasm_bindgen]
pub fn webclient_received_message(wcs: *mut WebClientState, msg: Vec<u8>) {
	panic!("TODO")
}

#[wasm_bindgen]
pub fn webclient_tick(wcs: *mut WebClientState, input_states: JsValue) -> Vec<JsValue> { // -> Vec<Cmd>
	vec![]
}
