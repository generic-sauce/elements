pub use std::sync::mpsc::{channel, Sender, Receiver, SendError, TryRecvError};
pub use std::time::{Duration, SystemTime, Instant};
pub use std::thread::{self, sleep};
pub use std::rc::Rc;
pub use std::net::{ToSocketAddrs, UdpSocket, SocketAddr};
pub use std::collections::HashMap;
pub use std::io::BufReader;
pub use std::fs::File;
pub use itertools::iproduct;

#[cfg(feature = "client")] pub use sfml::system::{Vector2i, Vector2f, Vector2u, SfBox, Clock, Time};
#[cfg(feature = "client")] pub use sfml::graphics::{RenderWindow, Texture, Color, RenderStates, RenderTarget, Shape, RectangleShape, CircleShape, Transformable, Shader as SfmlShader, Sprite, Rect, IntRect, FloatRect, Image, Text, Font, View, RenderTexture};
#[cfg(feature = "client")] pub use sfml::window::{Style, VideoMode, Event, Key, joystick, ContextSettings};
#[cfg(feature = "client")] pub use gilrs::{GamepadId, Gilrs};

pub use serde::{Serialize, Serializer, Deserialize, Deserializer, de::DeserializeOwned};
pub use bincode::{serialize, deserialize};

#[cfg(feature = "client")] pub mod pxp { // the pixels prelude, release into global scope later
	pub use pixels::{Error, Pixels, SurfaceTexture};
	pub use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
	pub use winit::event::{Event, VirtualKeyCode, WindowEvent};
	pub use winit::event_loop::{ControlFlow, EventLoop};
	pub use winit::window::Window;
	pub use winit_input_helper::WinitInputHelper;
}

pub use crate::server::*;
pub use crate::world::*;
pub use crate::world::player::{*, sensor::*};
pub use crate::world::tilemap::*;
pub use crate::world::fluidmap::*;
pub use crate::vec::*;
pub use crate::timed_loop::*;
pub use crate::net::*;
pub use crate::animation::*;
pub use crate::resource::res;

#[cfg(feature = "client")] pub use crate::client::*;
#[cfg(feature = "client")] pub use crate::world::client_world::*;
#[cfg(feature = "client")] pub use crate::draw_context::*;
#[cfg(feature = "client")] pub use crate::local::*;
#[cfg(feature = "client")] pub use crate::app::*;
#[cfg(feature = "client")] pub use crate::texture_state::*;
#[cfg(feature = "client")] pub use crate::shader_state::*;
#[cfg(feature = "client")] pub use crate::font_state::*;
#[cfg(feature = "client")] pub use crate::animation_state::*;
#[cfg(feature = "client")] pub use crate::input::*;
#[cfg(feature = "client")] pub use crate::window_vec::*;
#[cfg(feature = "client")] pub use crate::menu::*;
#[cfg(feature = "client")] pub use crate::px::*;
