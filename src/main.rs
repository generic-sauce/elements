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
				peripherals_update = Some(PeripheralsUpdate::from_winit_input(virtual_keycode, state));
			},
			win::Event::WindowEvent { event: win::WindowEvent::MouseInput { state, button, .. }, .. } => {
				peripherals_update = Some(PeripheralsUpdate::from_winit_input(button, state));
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
