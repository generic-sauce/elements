pub use {
	crate::{
		*,
		animation::*,
		rng::*,
		vec::*,
		input::*,
		resource::res,
		input::*,
		player::*,
		tilemap::*,
		fluidmap::*,
		skill::*,
		event::*,
		update::*,
		packet::*,
	},
	networking::prelude::*,
};

#[cfg(not(target_arch = "wasm32"))]
pub use crate::client_socket_native::*;
