use crate::prelude::*;

pub struct WebClient {
	socket: WebSocket,
	receiver: Receiver<Vec<u8>>,
	mode: WebClientMode,
}

pub enum WebClientMode {
	WaitingForGo,
	InGame { player_id: usize }
}

impl WebClient {
	pub fn new(server: &str) -> Self {
		let (sender, receiver) = channel();

		let client = WebClient {
			socket: WebSocket::new(&format!("ws://{}:{}", server, PORT)).unwrap(),
			receiver,
			mode: WebClientMode::WaitingForGo,
		};

		client.socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

		let cb = Closure::<dyn Fn(web_sys::MessageEvent)>::wrap(Box::new(move |ev| {
			let data: JsValue = ev.data();
			let data: js_sys::ArrayBuffer = data.dyn_into().unwrap();
			let data: Uint8Array = Uint8Array::new(&data);
			let data: Vec<u8> = data.to_vec();
			sender.send(data).unwrap();
		}));
		let leaked_cb = Box::leak(Box::new(cb)); // TODO
		client.socket.set_onmessage(Some(leaked_cb.as_ref().dyn_ref().unwrap()));

		client
	}


	fn handle_packets(&mut self, webapp_data: &mut WebappData) {
		match self.mode {
			WebClientMode::WaitingForGo => {
				let go_bytes = match self.receiver.try_recv() {
					Err(TryRecvError::Empty) => return,
					x => x.unwrap(),
				};
				let Go { your_player_id } = deser::<Go>(&go_bytes[..]);
				self.mode = WebClientMode::InGame { player_id: your_player_id };
				log("game is starting!");
			},
			WebClientMode::InGame { .. } => {
				loop {
					let update_bytes = match self.receiver.try_recv() {
						Err(TryRecvError::Empty) => return,
						x => x.unwrap(),
					};
					let update = deser::<WorldUpdate>(&update_bytes[..]);

					webapp_data.world.apply_update(update, &mut ());
				}
			},
		}
	}

	pub fn tick(&mut self, webapp_data: &mut WebappData) {
		self.handle_packets(webapp_data);
		if let WebClientMode::InGame { player_id } = self.mode {
			webapp_data.world.players[player_id].input = input_state(0);
			let input_bytes = ser(&webapp_data.world.players[player_id].input);
			self.socket.send_with_u8_array(&input_bytes[..]).unwrap();
			webapp_data.world.tick(&mut ());
		}
	}
}