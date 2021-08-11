use crate::prelude::*;

pub struct WebSocketBackend {
	socket: WebSocket,
	msg_receiver: Receiver<Vec<u8>>,
	_msg_closure: Closure<dyn Fn(web_sys::MessageEvent)>,
}

impl SocketBackend for WebSocketBackend {
	fn new(server_ip: &str, port: u16) -> Result<Self, SocketErr> {
		let ip_string = match server_ip.starts_with("http://") {
			true => format!("ws://{}:{}", server_ip.trim_start_matches("http://"), port),
			false => format!("wss://{}:{}", server_ip, port+1),
		};
		let socket = WebSocket::new(&ip_string).map_err(|_| strerr("WebSocket::new() failed"))?; // TODO better error management
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

		Ok(WebSocketBackend {
			socket,
			msg_receiver,
			_msg_closure: msg_closure,
		})
	}

	fn send(&mut self, packet: &impl Packet) -> Result<(), SocketErr> {
		if !matches!(self.socket.ready_state(), WebSocket::OPEN) {
			return Err(strerr("ERR: websocket-send: ready_state is not open!"));
		}

		let input_bytes = ser(packet)?;
		self.socket.send_with_u8_array(&input_bytes[..]).map_err(|_| strerr("WebSocket::send_with_u8_array failed"))?; // TODO better error management
		Ok(())
	}

	fn tick(&mut self) { }

	fn recv<P: Packet>(&mut self) -> Result<Option<P>, SocketErr> {
		if !matches!(self.socket.ready_state(), WebSocket::OPEN) {
			return Err(strerr("ERR: websocket-recv: ready_state is not open!"));
		}

		let bytes = match self.msg_receiver.try_recv() {
			Err(TryRecvError::Empty) => return Ok(None),
			x => x.unwrap(),
		};
		Ok(Some(deser::<P>(&bytes[..])?))
	}
}