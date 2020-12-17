#![feature(drain_filter)]
#![feature(const_fn)]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]

include!("base.rs");

use crate::prelude::*;

#[cfg(feature = "game")] const DEFAULT_CURSOR_POSITION: SubPixelVec = SubPixelVec::new(300.0, 300.0);

#[cfg(feature = "native-client")]
fn main_loop(mut f: impl FnMut(), fps: u32) {
	TimedLoop::with_fps(fps)
		.for_each(move |_| f());
}

#[cfg(feature = "native-client")]
fn main() {
	let matches = ClapApp::new("Elements Native Client")
		.about("This is the Native Client of the Elements Game. Have fun :D")
		.subcommand(SubCommand::with_name("server")
			.about("Starts the Elements game server")
			.arg(Arg::with_name("port")
				.short("-p")
				.long("--port")
				.value_name("PORT")
				.help(&format!("The server will bind this port. (default: {})", DEFAULT_GAME_SERVER_PORT))
				.takes_value(true)
			)
			.arg(Arg::with_name("domain_name")
				.short("-d")
				.long("--domain-name")
				.value_name("DOMAIN_NAME")
				.help(&"The domain name of this server. Only used, if connecting to a master server.")
				.takes_value(true)
			)
		)
		.subcommand(SubCommand::with_name("menu")
			.about("Starts the Elements Native Clients menu")
		)
		.subcommand(SubCommand::with_name("connect")
			.about("Connects to the following ip")
			.arg(Arg::with_name("server-ip")
				.help("The server ip to connect to")
				.required(true)
				.index(1)
			)
			.arg(Arg::with_name("port")
				.short("-p")
				.long("--port")
				.value_name("PORT")
				.help(&format!("The client will connect to this game server port. (default: {})", DEFAULT_GAME_SERVER_PORT))
				.takes_value(true)
			)
		)
		.subcommand(SubCommand::with_name("local")
			.about("Starts the game locally")
			.arg(Arg::with_name("best_of")
				.short("-n")
				.long("--best-of")
				.value_name("BEST_OF")
				.help("Defines the win condition of the match. The winner is the player who wins the most out of <best-of> games. (default: infinite)")
			)
		)
		.get_matches();

	if let Some(matches) = matches.subcommand_matches("server") {
		let port = matches.value_of("port")
			.map(|p| p.parse::<u16>().expect("Port argument seems not to be a valid port!"))
			.unwrap_or(DEFAULT_GAME_SERVER_PORT);
		let domain_name = matches.value_of("domain_name");
		Server::new(port, domain_name).run();
		return;
	}

	let (draw_sender, draw_receiver) = channel::<Draw>();
	let (peripherals_sender, peripherals_receiver) = channel::<PeripheralsUpdate>();

	thread::spawn(move || {
		let mut runnable = match matches.subcommand_name() {
			Some("menu") => Runnable::Menu,
			Some("connect") => {
				let matches = matches.subcommand_matches("connect").unwrap();
				let ip = matches.value_of("server-ip").unwrap();
				let port = matches.value_of("port")
					.map(|p| p.parse::<u16>().expect("Port argument seems not to be a valid port!"))
					.unwrap_or(DEFAULT_GAME_SERVER_PORT);
				Runnable::Client(Client::new(&ip, port))
			},
			Some("local") => {
				let matches = matches.subcommand_matches("local").unwrap();
				let best_of = matches.value_of("best_of").map(|n| n.parse::<u32>().expect("Value of best of is invalid!")).unwrap_or(0);
				Runnable::Local(Local::new(best_of))
			},
			_ => Runnable::Local(Local::new(0)),
		};
		let input_backend = NativeInputBackend::new(peripherals_receiver);
		let graphics_backend = NativeGraphicsBackend::new(draw_sender);
		let mut app = App::<NativeBackend>::new(graphics_backend, input_backend, runnable.build_menu());
		main_loop(move || app.tick_draw(&mut runnable), 60);
	});

	let event_loop = win::EventLoop::new();
	let window = win::WindowBuilder::new()
		.with_inner_size(win::PhysicalSize::new(1280, 720))
		// .with_resizable(false)
		.with_title("Elements")
		.build(&event_loop)
		.unwrap();

	let mut graphics = Graphics::new(&window);

	event_loop.run(move |event, _window_target, control_flow| {
		let next_frame_instant = Instant::now() + Duration::from_millis(1);
		*control_flow = win::ControlFlow::WaitUntil(next_frame_instant);

		let mut peripherals_update: Option<PeripheralsUpdate> = None;

		match event {
			win::Event::WindowEvent { event: win::WindowEvent::CloseRequested, .. } => {
				*control_flow = win::ControlFlow::Exit;
			},
			win::Event::WindowEvent { event: win::WindowEvent::CursorMoved { position, .. }, .. } => {
				let cursor_position = SubPixelVec::new(position.x as f32, position.y as f32);
				let cursor_move = cursor_position - DEFAULT_CURSOR_POSITION;
				if cursor_move.x != 0.0 || cursor_move.y != 0.0 {
					peripherals_update = Some(PeripheralsUpdate::MouseMove(cursor_move));
					window.set_cursor_position(win::PhysicalPosition { x: DEFAULT_CURSOR_POSITION.x as f64, y: DEFAULT_CURSOR_POSITION.y as f64 }).unwrap();
				}
			},
			win::Event::WindowEvent { event: win::WindowEvent::Resized(size), .. } => {
				graphics.resize(PixelVec::new(size.width, size.height));
			},
			win::Event::WindowEvent { event: win::WindowEvent::ReceivedCharacter(c), .. } => {
				peripherals_update = Some(PeripheralsUpdate::Text(Character::from(c)));
			},
			win::Event::WindowEvent { event: win::WindowEvent::KeyboardInput { input: win::KeyboardInput { virtual_keycode: Some(virtual_keycode), state, .. }, .. }, .. } => {
				peripherals_update = Some(from_winit_input(key_from(virtual_keycode), state));
			},
			win::Event::WindowEvent { event: win::WindowEvent::MouseInput { state, button, .. }, .. } => {
				peripherals_update = Some(from_winit_input(key_from_mouse(button), state));
			},
			win::Event::MainEventsCleared => {
				window.request_redraw();
			},
			win::Event::RedrawRequested { .. } => {
				let mut opt_draw = None;

				loop {
					match draw_receiver.try_recv() {
						Ok(draw) => opt_draw = Some(draw),
						Err(TryRecvError::Empty) => break,
						e @ Err(TryRecvError::Disconnected) => { e.unwrap(); },
					}
				}

				if let Some(draw) = opt_draw {
					graphics.render(draw);
				}
			},
			_ => ()
		}

		if let Some(update) = peripherals_update {
			peripherals_sender.send(update).unwrap();
		}

		// window.set_cursor_grab(true).unwrap();
		// window.set_cursor_visible(false);
	});
}

#[cfg(feature = "web-client")]
fn main() {
	panic!("web version does not have a main()!")
}

#[cfg(all(feature = "game-server", not(feature = "client")))]
fn main() {
	let matches = ClapApp::new("Elements Game Server")
		.about("This is the Game Server of the Elements Game. Lets host some game :D")
		.arg(Arg::with_name("port")
			.short("-p")
			.long("--port")
			.value_name("PORT")
			.help(&format!("The server will bind this port. (default: {})", DEFAULT_GAME_SERVER_PORT))
			.takes_value(true)
		)
		.arg(Arg::with_name("domain_name")
			.short("-d")
			.long("--domain-name")
			.value_name("DOMAIN_NAME")
			.help(&"The domain name of this server. Only used, if connecting to a master server.")
			.takes_value(true)
		)
		.get_matches();
	let port = matches.value_of("port")
		.map(|p| p.parse::<u16>().expect("Port argument seems not to be a valid port!"))
		.unwrap_or(DEFAULT_GAME_SERVER_PORT);
	let domain_name = matches.value_of("domain_name");
	Server::new(port, domain_name).run();
}

fn key_from(key_code: win::VirtualKeyCode) -> Key {
	match key_code {
		win::VirtualKeyCode::Key1 => Key::Key1,
		win::VirtualKeyCode::Key2 => Key::Key2,
		win::VirtualKeyCode::Key3 => Key::Key3,
		win::VirtualKeyCode::Key4 => Key::Key4,
		win::VirtualKeyCode::Key5 => Key::Key5,
		win::VirtualKeyCode::Key6 => Key::Key6,
		win::VirtualKeyCode::Key7 => Key::Key7,
		win::VirtualKeyCode::Key8 => Key::Key8,
		win::VirtualKeyCode::Key9 => Key::Key9,
		win::VirtualKeyCode::Key0 => Key::Key0,
		win::VirtualKeyCode::A => Key::A,
		win::VirtualKeyCode::B => Key::B,
		win::VirtualKeyCode::C => Key::C,
		win::VirtualKeyCode::D => Key::D,
		win::VirtualKeyCode::E => Key::E,
		win::VirtualKeyCode::F => Key::F,
		win::VirtualKeyCode::G => Key::G,
		win::VirtualKeyCode::H => Key::H,
		win::VirtualKeyCode::I => Key::I,
		win::VirtualKeyCode::J => Key::J,
		win::VirtualKeyCode::K => Key::K,
		win::VirtualKeyCode::L => Key::L,
		win::VirtualKeyCode::M => Key::M,
		win::VirtualKeyCode::N => Key::N,
		win::VirtualKeyCode::O => Key::O,
		win::VirtualKeyCode::P => Key::P,
		win::VirtualKeyCode::Q => Key::Q,
		win::VirtualKeyCode::R => Key::R,
		win::VirtualKeyCode::S => Key::S,
		win::VirtualKeyCode::T => Key::T,
		win::VirtualKeyCode::U => Key::U,
		win::VirtualKeyCode::V => Key::V,
		win::VirtualKeyCode::W => Key::W,
		win::VirtualKeyCode::X => Key::X,
		win::VirtualKeyCode::Y => Key::Y,
		win::VirtualKeyCode::Z => Key::Z,
		win::VirtualKeyCode::Escape => Key::Escape,
		win::VirtualKeyCode::F1 => Key::F1,
		win::VirtualKeyCode::F2 => Key::F2,
		win::VirtualKeyCode::F3 => Key::F3,
		win::VirtualKeyCode::F4 => Key::F4,
		win::VirtualKeyCode::F5 => Key::F5,
		win::VirtualKeyCode::F6 => Key::F6,
		win::VirtualKeyCode::F7 => Key::F7,
		win::VirtualKeyCode::F8 => Key::F8,
		win::VirtualKeyCode::F9 => Key::F9,
		win::VirtualKeyCode::F10 => Key::F10,
		win::VirtualKeyCode::F11 => Key::F11,
		win::VirtualKeyCode::F12 => Key::F12,
		win::VirtualKeyCode::F13 => Key::F13,
		win::VirtualKeyCode::F14 => Key::F14,
		win::VirtualKeyCode::F15 => Key::F15,
		win::VirtualKeyCode::F16 => Key::F16,
		win::VirtualKeyCode::F17 => Key::F17,
		win::VirtualKeyCode::F18 => Key::F18,
		win::VirtualKeyCode::F19 => Key::F19,
		win::VirtualKeyCode::F20 => Key::F20,
		win::VirtualKeyCode::F21 => Key::F21,
		win::VirtualKeyCode::F22 => Key::F22,
		win::VirtualKeyCode::F23 => Key::F23,
		win::VirtualKeyCode::F24 => Key::F24,
		win::VirtualKeyCode::Snapshot => Key::Snapshot,
		win::VirtualKeyCode::Scroll => Key::Scroll,
		win::VirtualKeyCode::Pause => Key::Pause,
		win::VirtualKeyCode::Insert => Key::Insert,
		win::VirtualKeyCode::Home => Key::Home,
		win::VirtualKeyCode::Delete => Key::Delete,
		win::VirtualKeyCode::End => Key::End,
		win::VirtualKeyCode::PageDown => Key::PageDown,
		win::VirtualKeyCode::PageUp => Key::PageUp,
		win::VirtualKeyCode::Left => Key::Left,
		win::VirtualKeyCode::Up => Key::Up,
		win::VirtualKeyCode::Right => Key::Right,
		win::VirtualKeyCode::Down => Key::Down,
		win::VirtualKeyCode::Back => Key::Back,
		win::VirtualKeyCode::Return => Key::Return,
		win::VirtualKeyCode::Space => Key::Space,
		win::VirtualKeyCode::Compose => Key::Compose,
		win::VirtualKeyCode::Caret => Key::Caret,
		win::VirtualKeyCode::Numlock => Key::Numlock,
		win::VirtualKeyCode::Numpad0 => Key::Numpad0,
		win::VirtualKeyCode::Numpad1 => Key::Numpad1,
		win::VirtualKeyCode::Numpad2 => Key::Numpad2,
		win::VirtualKeyCode::Numpad3 => Key::Numpad3,
		win::VirtualKeyCode::Numpad4 => Key::Numpad4,
		win::VirtualKeyCode::Numpad5 => Key::Numpad5,
		win::VirtualKeyCode::Numpad6 => Key::Numpad6,
		win::VirtualKeyCode::Numpad7 => Key::Numpad7,
		win::VirtualKeyCode::Numpad8 => Key::Numpad8,
		win::VirtualKeyCode::Numpad9 => Key::Numpad9,
		win::VirtualKeyCode::NumpadComma => Key::NumpadComma,
		win::VirtualKeyCode::NumpadEnter => Key::NumpadEnter,
		win::VirtualKeyCode::NumpadEquals => Key::NumpadEquals,
		win::VirtualKeyCode::Apostrophe => Key::Apostrophe,
		win::VirtualKeyCode::Apps => Key::Apps,
		win::VirtualKeyCode::At => Key::At,
		win::VirtualKeyCode::Ax => Key::Ax,
		win::VirtualKeyCode::Backslash => Key::Backslash,
		win::VirtualKeyCode::Calculator => Key::Calculator,
		win::VirtualKeyCode::Capital => Key::Capital,
		win::VirtualKeyCode::Colon => Key::Colon,
		win::VirtualKeyCode::Comma => Key::Comma,
		win::VirtualKeyCode::Convert => Key::Convert,
		win::VirtualKeyCode::Equals => Key::Equals,
		win::VirtualKeyCode::Grave => Key::Grave,
		win::VirtualKeyCode::Kana => Key::Kana,
		win::VirtualKeyCode::Kanji => Key::Kanji,
		win::VirtualKeyCode::LAlt => Key::LAlt,
		win::VirtualKeyCode::LBracket => Key::LBracket,
		win::VirtualKeyCode::LControl => Key::LControl,
		win::VirtualKeyCode::LShift => Key::LShift,
		win::VirtualKeyCode::LWin => Key::LWin,
		win::VirtualKeyCode::Mail => Key::Mail,
		win::VirtualKeyCode::MediaSelect => Key::MediaSelect,
		win::VirtualKeyCode::MediaStop => Key::MediaStop,
		win::VirtualKeyCode::Minus => Key::Minus,
		win::VirtualKeyCode::Mute => Key::Mute,
		win::VirtualKeyCode::MyComputer => Key::MyComputer,
		win::VirtualKeyCode::NavigateForward => Key::NavigateForward,
		win::VirtualKeyCode::NavigateBackward => Key::NavigateBackward,
		win::VirtualKeyCode::NextTrack => Key::NextTrack,
		win::VirtualKeyCode::NoConvert => Key::NoConvert,
		win::VirtualKeyCode::OEM102 => Key::OEM102,
		win::VirtualKeyCode::Period => Key::Period,
		win::VirtualKeyCode::PlayPause => Key::PlayPause,
		win::VirtualKeyCode::Power => Key::Power,
		win::VirtualKeyCode::PrevTrack => Key::PrevTrack,
		win::VirtualKeyCode::RAlt => Key::RAlt,
		win::VirtualKeyCode::RBracket => Key::RBracket,
		win::VirtualKeyCode::RControl => Key::RControl,
		win::VirtualKeyCode::RShift => Key::RShift,
		win::VirtualKeyCode::RWin => Key::RWin,
		win::VirtualKeyCode::Semicolon => Key::Semicolon,
		win::VirtualKeyCode::Slash => Key::Slash,
		win::VirtualKeyCode::Sleep => Key::Sleep,
		win::VirtualKeyCode::Stop => Key::Stop,
		win::VirtualKeyCode::Sysrq => Key::Sysrq,
		win::VirtualKeyCode::Tab => Key::Tab,
		win::VirtualKeyCode::Underline => Key::Underline,
		win::VirtualKeyCode::Unlabeled => Key::Unlabeled,
		win::VirtualKeyCode::VolumeDown => Key::VolumeDown,
		win::VirtualKeyCode::VolumeUp => Key::VolumeUp,
		win::VirtualKeyCode::Wake => Key::Wake,
		win::VirtualKeyCode::WebBack => Key::WebBack,
		win::VirtualKeyCode::WebFavorites => Key::WebFavorites,
		win::VirtualKeyCode::WebForward => Key::WebForward,
		win::VirtualKeyCode::WebHome => Key::WebHome,
		win::VirtualKeyCode::WebRefresh => Key::WebRefresh,
		win::VirtualKeyCode::WebSearch => Key::WebSearch,
		win::VirtualKeyCode::WebStop => Key::WebStop,
		win::VirtualKeyCode::Yen => Key::Yen,
		win::VirtualKeyCode::Copy => Key::Copy,
		win::VirtualKeyCode::Paste => Key::Paste,
		win::VirtualKeyCode::Cut => Key::Cut,
		_ => Key::Unknown,
	}
}

fn key_from_mouse(key_code: win::MouseButton) -> Key {
	match key_code {
		win::MouseButton::Left => Key::LeftMouse,
		win::MouseButton::Right => Key::RightMouse,
		win::MouseButton::Middle => Key::MiddleMouse,
		win::MouseButton::Other(id) => Key::OtherMouse(id),
	}
}

pub fn from_winit_input(key_source: impl Into<Key>, state: win::ElementState) -> PeripheralsUpdate {
	match state {
		win::ElementState::Pressed => PeripheralsUpdate::KeyPress(key_source.into()),
		win::ElementState::Released => PeripheralsUpdate::KeyRelease(key_source.into()),
	}
}
