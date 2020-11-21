use crate::prelude::*;

pub struct WebSocketBackend {
	socket: WebSocket,
	msg_receiver: Receiver<Vec<u8>>,
	_msg_closure: Closure<dyn Fn(web_sys::MessageEvent)>,
}

impl SocketBackend for WebSocketBackend {
	fn new(server_ip: &str) -> Self {
		let ip_string = match server_ip.starts_with("http://") {
			true => format!("ws://{}:{}", server_ip.trim_start_matches("http://"), PORT),
			false => format!("wss://{}:{}", server_ip, HTTPS_PORT),
		};
		let socket = WebSocket::new(&ip_string).unwrap();
		socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

		// message-closure
		let (msg_sender, msg_receiver) = channel();
		let msg_closure = Closure::<dyn Fn(web_sys::MessageEvent)>::wrap(Box::new(move |ev| {
			let data: JsValue = ev.data();
			let data: js_sys::ArrayBuffer = data.dyn_into().unwrap();
			let data: Uint8Array = Uint8Array::new(&data);
			let data: Vec<u8> = data.to_vec();
			msg_sender.send(data).unwrap();
		}));
		socket.set_onmessage(Some(msg_closure.as_ref().dyn_ref().unwrap()));

		WebSocketBackend {
			socket,
			msg_receiver,
			_msg_closure: msg_closure,
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

		let bytes = match self.msg_receiver.try_recv() {
			Err(TryRecvError::Empty) => return None,
			x => x.unwrap(),
		};
		Some(deser::<P>(&bytes[..]))
	}
}