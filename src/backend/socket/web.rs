use crate::prelude::*;

pub struct WebSocketBackend {
	socket: WebSocket,
	receiver: Receiver<Vec<u8>>,
}

impl SocketBackend for WebSocketBackend {
	fn new(server_ip: &str) -> Self {
		let (sender, receiver) = channel();

		let socket = WebSocket::new(&format!("ws://{}:{}", server_ip, PORT)).unwrap();
		socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

		let cb = Closure::<dyn Fn(web_sys::MessageEvent)>::wrap(Box::new(move |ev| {
			let data: JsValue = ev.data();
			let data: js_sys::ArrayBuffer = data.dyn_into().unwrap();
			let data: Uint8Array = Uint8Array::new(&data);
			let data: Vec<u8> = data.to_vec();
			sender.send(data).unwrap();
		}));
		let leaked_cb = Box::leak(Box::new(cb)); // TODO
		socket.set_onmessage(Some(leaked_cb.as_ref().dyn_ref().unwrap()));

		WebSocketBackend {
			socket,
			receiver,
		}
	}

	fn send(&mut self, packet: &impl Packet) {
		let input_bytes = ser(packet);
		self.socket.send_with_u8_array(&input_bytes[..]).unwrap();
	}

	fn try_recv<P: Packet>(&mut self) -> Option<P> {
		let bytes = match self.receiver.try_recv() {
			Err(TryRecvError::Empty) => return None,
			x => x.unwrap(),
		};
		Some(deser::<P>(&bytes[..]))
	}
}