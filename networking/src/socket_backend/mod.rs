use crate::prelude::*;

use std::error::Error;
use std::fmt::{self, Display, Debug};

#[cfg(not(target_arch = "wasm32"))] mod native;
#[cfg(not(target_arch = "wasm32"))] pub use native::*;

pub type SocketErr = Box<dyn Error>;

pub trait SocketBackend {
	fn new(server_ip: &str, port: u16) -> Result<Self, SocketErr> where Self: Sized;
	fn send(&mut self, packet: &impl Packet) -> Result<(), SocketErr>;
	fn tick(&mut self); // does this also want to return Result<(), SocketErr>?
	fn recv<P: Packet>(&mut self) -> Result<Option<P>, SocketErr>;
}

pub struct StringErr(String);

impl Display for StringErr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", &self.0)
	}
}

impl Debug for StringErr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", &self.0)
	}
}

impl Error for StringErr {}

pub fn strerr(s: impl Into<String>) -> SocketErr {
	Box::new(StringErr(s.into()))
}
