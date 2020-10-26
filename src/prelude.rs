pub use std::sync::mpsc::{channel, Sender, Receiver, SendError, TryRecvError};
pub use std::time::{Duration, SystemTime, Instant};
pub use std::thread::{self, sleep};
pub use std::rc::Rc;
pub use std::net::{ToSocketAddrs, UdpSocket, SocketAddr, TcpStream, TcpListener};
pub use std::collections::HashMap;
pub use std::io::BufReader;
pub use std::fs::File;
pub use itertools::iproduct;
pub use std::any::Any;
pub use std::io::ErrorKind;

#[cfg(feature = "native-client")] pub use sfml::system::{Vector2i, Vector2f, Vector2u, SfBox, Clock, Time};
#[cfg(feature = "native-client")] pub use sfml::graphics::{RenderWindow, Texture, Color, RenderStates, RenderTarget, Shape, RectangleShape, CircleShape, Transformable, Shader as SfmlShader, Sprite, Rect, IntRect, FloatRect, Image, Text, Font, View, RenderTexture};
#[cfg(feature = "native-client")] pub use sfml::window::{Style, VideoMode, Event, Key, joystick, ContextSettings};
#[cfg(feature = "native-client")] pub use gilrs::{GamepadId, Gilrs};

pub use serde::{Serialize, Serializer, Deserialize, Deserializer, de::DeserializeOwned};
pub use bincode::{serialize, deserialize};

pub use crate::rng::*;
pub use crate::world::*;
pub use crate::world::player::{*, sensor::*};
pub use crate::world::tilemap::*;
pub use crate::world::fluidmap::*;
pub use crate::vec::*;
pub use crate::animation::*;
pub use crate::input::*;

#[cfg(feature = "server")] pub type TungSocket = tungstenite::WebSocket<TcpStream>;
#[cfg(feature = "server")] pub use tungstenite::Message;

pub use crate::net::*;
#[cfg(not(feature = "web-client"))] pub use crate::resource::res;
#[cfg(not(feature = "web-client"))] pub use crate::timed_loop::*;
#[cfg(not(feature = "web-client"))] pub use crate::server::*;

#[cfg(feature = "web-client")] pub use crate::web::*;
#[cfg(feature = "web-client")] pub use wasm_bindgen::{prelude::*, JsCast};
#[cfg(feature = "web-client")] pub use web_sys::{WebSocket};
#[cfg(feature = "web-client")] pub use js_sys::{Uint8Array};

#[cfg(feature = "native-client")] pub use crate::client::*;
#[cfg(feature = "native-client")] pub use crate::client_world::*;
#[cfg(feature = "native-client")] pub use crate::draw_context::*;
#[cfg(feature = "native-client")] pub use crate::draw::*;
#[cfg(feature = "native-client")] pub use crate::local::*;
#[cfg(feature = "native-client")] pub use crate::app::*;
#[cfg(feature = "native-client")] pub use crate::texture_state::*;
#[cfg(feature = "native-client")] pub use crate::shader_state::*;
#[cfg(feature = "native-client")] pub use crate::font_state::*;
#[cfg(feature = "native-client")] pub use crate::animation_state::*;
#[cfg(feature = "native-client")] pub use crate::window_vec::*;
#[cfg(feature = "native-client")] pub use crate::menu::*;
#[cfg(feature = "native-client")] pub use crate::graphics::*;
#[cfg(feature = "native-client")] pub use crate::graphics::world::*;
#[cfg(feature = "native-client")] pub use crate::graphics::surface_vec::*;
#[cfg(feature = "native-client")] pub use crate::graphics::texture_state2::*;
#[cfg(feature = "native-client")] pub use winit_input_helper::WinitInputHelper;
#[cfg(feature = "native-client")] pub mod win {
	pub use winit::{
			dpi::{ LogicalPosition, LogicalSize, PhysicalSize, },
			window::{ Window, WindowBuilder, },
			event::{ Event, VirtualKeyCode, WindowEvent, ScanCode, KeyboardInput },
			event_loop::{ EventLoop, ControlFlow, },
	};
}
