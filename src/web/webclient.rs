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
	InGame { player_id: usize }
}

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

		let cb = Closure::<dyn Fn(web_sys::MessageEvent)>::wrap(Box::new(move |ev| {
			sender.send(vec![]).unwrap(); // TODO
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
		while self.fps() < FPS {
			self.tick();
			self.tick_counter += 1;
		}
		self.draw();
	}

	pub fn tick(&mut self) {
		if let WebClientState::InGame { .. } = self.state {
			self.world.tick(&mut ());
		}
	}

	pub fn draw(&self) {
		RenderWorld::draw(&self.world);
	}
}