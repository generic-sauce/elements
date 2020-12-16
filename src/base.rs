#[macro_use]
extern crate serde_derive;

// #[macro_use]
// mod fps_timer;

#[cfg(feature = "client")] mod backend;
#[cfg(feature = "client")] mod draw;
#[cfg(feature = "client")] mod client;
#[cfg(feature = "client")] mod server_connector;
#[cfg(feature = "client")] mod app;
#[cfg(feature = "client")] mod local;
#[cfg(feature = "client")] mod menu;
#[cfg(feature = "client")] mod timer;

#[cfg(feature = "native-client")] mod graphics;

#[cfg(feature = "web-client")] mod web;

// game-server (or native-client)
#[cfg(feature = "game-server")] mod server;
#[cfg(feature = "server_feature")] mod native_socket_backend;
#[cfg(feature = "game")] mod resource;


mod net;
#[cfg(feature = "game")] mod rng;
#[cfg(feature = "game")] mod world;
#[cfg(feature = "game")] mod vec;
#[cfg(feature = "game")] mod animation;
mod prelude;
#[cfg(feature = "game")] mod input;
