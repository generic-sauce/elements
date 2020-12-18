#[macro_use]
extern crate serde_derive;

// #[macro_use]
// mod fps_timer;

#[cfg(feature = "client_feature")] mod backend;
#[cfg(feature = "client_feature")] mod timer;

#[cfg(feature = "native-client")] mod graphics;

#[cfg(feature = "web-client")] mod web;

mod prelude;
