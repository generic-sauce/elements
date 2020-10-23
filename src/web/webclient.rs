use crate::prelude::*;

const FPS: f64 = 60.0;

pub struct WebClient {
	world: World,
	state: WebClientState,
	tick_counter: u32,
	start_time: f64,
	socket: WebSocket,
	receiver: Receiver<Vec<u8>>,
}

pub enum WebClientState {
	WaitingForGo,
	InGame { player_id: usize } }

impl WebClient {
	pub fn new(server: &'static str, src: TileMapImage) -> Self {
		let (sender, receiver) = channel();

		let client = WebClient {
			world: World::new(0, src),
			state: WebClientState::WaitingForGo,
			tick_counter: 0,
			start_time: now(),
			socket: WebSocket::new(&format!("ws://{}:{}", server, PORT)).unwrap(),
			receiver,
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

	pub fn schedule(mut self) {
		let cb = Closure::<dyn FnMut()>::wrap(Box::new(move || { self.run_once(); }));
		let leaked_cb = Box::leak(Box::new(cb)); // TODO
		setInterval(leaked_cb, 1000.0/FPS);
	}

	fn fps(&self) -> f64 {
		self.tick_counter as f64 * 1000.0 / (now() - self.start_time)
	}

	fn run_once(&mut self) {
		for _ in 0..10 {
			if self.fps() >= FPS { break; }

			self.tick();
			self.tick_counter += 1;
		}
		self.draw();
	}

	pub fn tick(&mut self) {
		match self.state {
			WebClientState::WaitingForGo => {
				let go_bytes = match self.receiver.try_recv() {
					Err(TryRecvError::Empty) => return,
					x => x.unwrap(),
				};
				let Go { your_player_id } = deser::<Go>(&go_bytes[..]);
				self.state = WebClientState::InGame { player_id: your_player_id };
				log("game is starting!");
			},
			WebClientState::InGame { .. } => self.world.tick(&mut ()),
		}
	}

	pub fn draw(&self) {
		RenderWorld::draw(&self.world);
	}
}