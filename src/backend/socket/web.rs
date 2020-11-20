use crate::prelude::*;

pub struct WebSocketBackend {
	socket: WebSocket,
	receiver: Receiver<Vec<u8>>,
	_closure: Closure<dyn Fn(web_sys::MessageEvent)>,
}

impl SocketBackend for WebSocketBackend {
	fn new(server_ip: &str) -> Self {
		let (sender, receiver) = channel();

		let socket = WebSocket::new(&format!("wss://{}:{}", server_ip, PORT)).unwrap();
		socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

		let closure = Closure::<dyn Fn(web_sys::MessageEvent)>::wrap(Box::new(move |ev| {
			let data: JsValue = ev.data();
			let data: js_sys::ArrayBuffer = data.dyn_into().unwrap();
			let data: Uint8Array = Uint8Array::new(&data);
			let data: Vec<u8> = data.to_vec();
			sender.send(data).unwrap();
		}));

		socket.set_onmessage(Some(closure.as_ref().dyn_ref().unwrap()));

		WebSocketBackend {
			socket,
			receiver,
			_closure: closure,
		}
	}

	fn is_open(&self) -> bool {
		self.socket.ready_state() == WebSocket::OPEN
	}

	fn send(&mut self, packet: &impl Packet) {
		assert_eq!(self.socket.ready_state(), WebSocket::OPEN);

		let input_bytes = ser(packet);
		self.socket.send_with_u8_array(&input_bytes[..]).unwrap();
	}

	fn try_recv<P: Packet>(&mut self) -> Option<P> {
		assert_eq!(self.socket.ready_state(), WebSocket::OPEN);

		let bytes = match self.receiver.try_recv() {
			Err(TryRecvError::Empty) => return None,
			x => x.unwrap(),
		};
		Some(deser::<P>(&bytes[..]))
	}
}