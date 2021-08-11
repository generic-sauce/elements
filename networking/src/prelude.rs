pub static DEFAULT_MASTER_SERVER_HOSTNAME: &str = "generic-sauce.de";
pub static AVAILABLE_MAPS: &'static[&'static str] = &["map01.png", "map02.png", "map03.png", "map04.png"];

pub use serde::{Serialize, Serializer, Deserialize, Deserializer, de::DeserializeOwned};
pub use serde_derive::*;
pub use bincode::{serialize, deserialize};
pub use std::{
	net::{ToSocketAddrs, UdpSocket, SocketAddr, TcpStream, TcpListener},
	io::{Read, Write, BufReader, ErrorKind},
	time::{Duration, SystemTime, Instant},
	thread::{self, sleep},
	fs::{File, read},
	sync::mpsc::{channel, Sender, Receiver, SendError, TryRecvError},
	rc::Rc,
	collections::{HashMap, HashSet},
	any::Any,
	marker::PhantomData,
	cmp::Ordering,
	ops::Mul,
	sync::Arc,
};
pub use itertools::iproduct;

#[cfg(not(target_arch = "wasm32"))] pub use native_tls::{TlsAcceptor, Identity};
#[cfg(not(target_arch = "wasm32"))] pub use tungstenite::Message;

pub use crate::{*,
	packets::*,
	constants::*,
	socket_backend::*,
};

#[cfg(not(target_arch = "wasm32"))] pub use timed_loop::*;
#[cfg(not(target_arch = "wasm32"))] pub use udp::*;
#[cfg(not(target_arch = "wasm32"))] pub use peer::*;
#[cfg(not(target_arch = "wasm32"))] pub use fps_timer::*;
