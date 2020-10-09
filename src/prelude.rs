pub use std::sync::mpsc::{channel, Sender, Receiver, SendError, TryRecvError};
pub use std::thread::{self};
pub use std::rc::Rc;
pub use std::net::{ToSocketAddrs, UdpSocket, SocketAddr};
pub use std::collections::HashMap;
pub use std::io::BufReader;
pub use std::fs::File;

pub use serde::{Serialize, Serializer, Deserialize, Deserializer, de::DeserializeOwned};
pub use bincode::{serialize, deserialize};

pub use crate::world::*;
pub use crate::world::player::{*, sensor::*};
pub use crate::world::tilemap::*;
pub use crate::world::fluidmap::*;
pub use crate::vec::*;

pub use crate::local::*;
pub use crate::input::*;
pub use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
	pub fn alert(s: &str);
	pub fn draw_world(w: &JsValue);
}
