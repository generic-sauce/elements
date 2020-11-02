#[macro_use]
extern crate serde_derive;

#[cfg(feature = "client")] #[macro_use] extern crate lazy_static;
#[cfg(feature = "client")] mod backend;
#[cfg(feature = "client")] mod draw;

#[cfg(feature = "native-client")] mod client;
#[cfg(feature = "native-client")] mod app;
#[cfg(feature = "native-client")] mod local;
#[cfg(feature = "native-client")] mod menu;
#[cfg(feature = "native-client")] mod graphics;

#[cfg(feature = "web-client")] mod web;

// server (or native-client)
#[cfg(feature = "server")] mod server;
#[cfg(feature = "server")] mod resource;
#[cfg(feature = "server")] mod timed_loop;

#[macro_use]
mod fps_timer;

mod net;
mod rng;
mod world;
mod vec;
mod animation;
mod prelude;
mod input;
