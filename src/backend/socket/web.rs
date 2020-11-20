use crate::prelude::*;

pub struct WebSocketBackend {
	server_ip: String,
	https: bool,
	socket: WebSocket,
	msg_receiver: Receiver<Vec<u8>>,
	err_receiver: Receiver<()>,
	_msg_closure: Closure<dyn Fn(web_sys::MessageEvent)>,
	_err_closure: Closure<dyn Fn()>,
}

impl WebSocketBackend {
	fn new_by_protocol(server_ip: &str, https: bool) -> Self {
		let (protocol, port) = match https {
			true  => ("wss", HTTPS_PORT),
			false => ("ws", PORT),
		};
		let socket = WebSocket::new(&format!("{}://{}:{}", protocol, server_ip, port)).unwrap();
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

		// err-closure
		let (err_sender, err_receiver) = channel();
		let err_closure = Closure::<dyn Fn()>::wrap(Box::new(move || {
			err_sender.send(()).unwrap();
		}));
		socket.set_onerror(Some(err_closure.as_ref().dyn_ref().unwrap()));

		WebSocketBackend {
			server_ip: server_ip.to_owned(),
			https,
			socket,
			msg_receiver,
			err_receiver,
			_msg_closure: msg_closure,
			_err_closure: err_closure,
		}
	}
}

impl SocketBackend for WebSocketBackend {
	fn new(server_ip: &str) -> Self {
		Self::new_by_protocol(server_ip, true)
	}

	fn is_open(&mut self) -> bool {
		if self.socket.ready_state() == WebSocket::OPEN { return true; }

		match self.err_receiver.try_recv() {
			// the err_receiver received something!
			Ok(_) => {
				if self.https {
					// fallback to http
					self.socket.close().unwrap();
					*self = Self::new_by_protocol(&self.server_ip, false);
				} else {
					panic!("Could not connect even with http");
				}
			}

			e @ Err(TryRecvError::Disconnected) => {
				e.unwrap();
				unreachable!()
			},

			Err(TryRecvError::Empty) => {}, // waiting...
		}

		false
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