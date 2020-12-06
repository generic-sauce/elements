// generic

pub use {
	serde::{Serialize, Serializer, Deserialize, Deserializer, de::DeserializeOwned},
	std::{
		net::{ToSocketAddrs, UdpSocket, SocketAddr, TcpStream, TcpListener},
		io::{Read, BufReader, ErrorKind},
		time::{Duration, SystemTime, Instant},
		thread::{self, sleep},
		fs::File,
	},
	bincode::{serialize, deserialize},
	crate::{
		net::*,
	},
};

#[cfg(feature = "game")]
pub use {
	std::{
		sync::mpsc::{channel, Sender, Receiver, SendError, TryRecvError},
		rc::Rc,
		collections::{HashMap, HashSet},
		any::Any,
		marker::PhantomData,
		cmp::Ordering,
		ops::Mul,
	},
	itertools::iproduct,
	crate::{
		animation::*,
		rng::*,
		world::*,
		vec::*,
		input::*,
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

#[cfg(feature = "game-server")] pub use {
	crate::{
		server::*,
	},
};

// game-server (or native-client)
#[cfg(feature = "server")] pub type TungSocket = tungstenite::WebSocket<TcpStream>;
#[cfg(feature = "server")] pub type TungTlsSocket = tungstenite::WebSocket<native_tls::TlsStream<TcpStream>>;
#[cfg(feature = "server")] pub use {
	std::sync::Arc,
	tungstenite::{protocol::Role, Message},
	native_tls::{Identity, TlsAcceptor},
	crate::{
		timed_loop::*,
		peer::*,
	}
};

#[cfg(feature = "master-server")] pub use {
	crate::master_server::*,
};