// generic
pub use {
	std::{
		sync::mpsc::{channel, Sender, Receiver, SendError, TryRecvError},
		time::{Duration, SystemTime, Instant},
		thread::{self, sleep},
		rc::Rc,
		net::{ToSocketAddrs, UdpSocket, SocketAddr, TcpStream, TcpListener},
		collections::{HashMap, HashSet},
		io::{BufReader, ErrorKind},
		fs::File,
		any::Any,
		marker::PhantomData,
		cmp::Ordering,
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
	},
};

// *-client
#[cfg(feature = "client")] pub use {
	crate::{
		draw::*,
	},
};

// native-client
#[cfg(feature = "native-client")] pub use {
	gilrs::{GamepadId, Gilrs},
	crate::{
		client::*,
		local::*,
		app::*,
		menu::*,
		graphics::*,
		backend::*,
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
#[cfg(feature = "server")] pub type TungSocket = tungstenite::WebSocket<TcpStream>;
#[cfg(feature = "server")] pub use {
	tungstenite::Message,
	crate::{
		resource::res,
		timed_loop::*,
		server::*
	}
};
