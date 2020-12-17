// generic

pub use {
	crate::{
		net::*,
	},
	networking::prelude::*,
};

#[cfg(feature = "game")]
pub use {
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
		server_connector::*,
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
#[cfg(feature = "game-server")] pub use clap::{App as ClapApp, Arg, SubCommand};

// game-server (or native-client)
#[cfg(feature = "server_feature")] pub use {
	std::sync::Arc,
	tungstenite::{protocol::Role, Message},
	native_tls::{Identity, TlsAcceptor},
	crate::{
		native_socket_backend::*,
	},
	native_utils::prelude::*,
};
