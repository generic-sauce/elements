// generic
pub use {
	std::{
		sync::mpsc::{channel, Sender, Receiver, SendError, TryRecvError},
		time::{Duration, SystemTime, Instant},
		thread::{self, sleep},
		rc::Rc,
		net::{ToSocketAddrs, UdpSocket, SocketAddr, TcpStream, TcpListener},
		collections::{HashMap, HashSet},
		io::{Read, BufReader, ErrorKind},
		fs::File,
		any::Any,
		marker::PhantomData,
		cmp::Ordering,
		ops::Mul,
	},
	itertools::iproduct,
	serde::{Serialize, Serializer, Deserialize, Deserializer, de::DeserializeOwned},
	bincode::{serialize, deserialize},
	crate::{
		rng::*,
		world::*,
		vec::*,
		animation::*,
		input::*,
		net::*,
		resource::res,
	},
};

// *-client
#[cfg(feature = "client")] pub use {
	crate::{
		draw::*,
		client::*,
		local::*,
		app::*,
		menu::*,
		backend::*,
		timer::*,
	},
};

// native-client
#[cfg(feature = "native-client")] pub use {
	gilrs::{GamepadId, Gilrs},
	crate::{
		graphics::*,
	},
};
#[cfg(feature = "native-client")] pub mod win {
	pub use winit::{
		dpi::{ LogicalPosition, LogicalSize, PhysicalSize, PhysicalPosition, },
		window::{ Window, WindowBuilder, },
		event::{ Event, VirtualKeyCode, WindowEvent, ScanCode, KeyboardInput, ElementState, MouseButton },
		event_loop::{ EventLoop, ControlFlow, },
	};
}

// web-client
#[cfg(feature = "web-client")] pub use {
	wasm_bindgen::{prelude::*, JsCast},
	web_sys::{WebSocket},
	js_sys::{Uint8Array},
	crate::{
		web::*,
		backend::*,
	},
};

// server (or native-client)
#[cfg(feature = "server")] pub type TungSocket = tungstenite::WebSocket<native_tls::TlsStream<TcpStream>>;
#[cfg(feature = "server")] pub use {
	std::sync::Arc,
	tungstenite::Message,
	native_tls::{Identity, TlsAcceptor},
	crate::{
		timed_loop::*,
		server::*
	}
};
