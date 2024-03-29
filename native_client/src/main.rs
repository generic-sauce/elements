mod backend;
mod graphics;
mod timer;
mod winit_inputs;

mod prelude;

use crate::prelude::*;

fn main_loop(mut f: impl FnMut(), fps: u32) {
	TimedLoop::with_fps(fps)
		.for_each(move |_| f());
}

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
			.arg(Arg::with_name("identity_file")
				.short("-i")
				.long("--identity-file")
				.value_name("IDENTITY_FILE")
				.help(&"The identity file for tls. If not given https is not supported")
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

		let identity_file = matches.value_of("identity_file");

		Server::new(port, domain_name, identity_file).run();
		return;
	}

	let (draw_sender, draw_receiver) = channel::<Draw>();
	let (peripherals_sender, peripherals_receiver) = channel::<PeripheralsUpdate>();

	thread::spawn(move || {
		let mut runnable = match matches.subcommand_name() {
			Some("connect") => {
				let matches = matches.subcommand_matches("connect").unwrap();
				let ip = matches.value_of("server-ip").unwrap();
				let port = matches.value_of("port")
					.map(|p| p.parse::<u16>().expect("Port argument seems not to be a valid port!"))
					.unwrap_or(DEFAULT_GAME_SERVER_PORT);
				Runnable::Client(Client::new(&ip, port).unwrap())
			},
			Some("local") => {
				let matches = matches.subcommand_matches("local").unwrap();
				let best_of = matches.value_of("best_of").map(|n| n.parse::<u32>().expect("Value of best of is invalid!")).unwrap_or(0);
				Runnable::Local(Local::new(best_of))
			},
			_ => Runnable::OnlineMenu(OnlineMenu::new()),
		};
		let input_backend = NativeInputBackend::new(peripherals_receiver);
		let graphics_backend = NativeGraphicsBackend::new(draw_sender);
		let storage_backend = NativeStorageBackend::new();
		let mut app = App::<NativeBackend>::new(graphics_backend, input_backend, storage_backend, DEFAULT_MASTER_SERVER_HOSTNAME);
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

	let mut focused = true;
	window.set_cursor_visible(!focused);

	let mut mouse_update_fix = SubPixelVec::new(0.0, 0.0);

	event_loop.run(move |event, _window_target, control_flow| {
		let next_frame_instant = Instant::now() + Duration::from_millis(1);
		*control_flow = win::ControlFlow::WaitUntil(next_frame_instant);

		let mut peripherals_update: Option<PeripheralsUpdate> = None;


		match event {
			win::Event::WindowEvent { event: win::WindowEvent::CloseRequested, .. } => {
				*control_flow = win::ControlFlow::Exit;
			},
			win::Event::WindowEvent { event: win::WindowEvent::CursorMoved { position, .. }, .. } => {
				if focused {
					let cursor_position = SubPixelVec::new(position.x as f32, position.y as f32);
					let window_size = window.inner_size();
					let window_size = SubPixelVec::new(window_size.width as f32, window_size.height as f32);
					let window_center = ViewVec::new(0.5, 0.5).to_subpixel(window_size).trunc();
					let cursor_move = cursor_position - window_center;
					if cursor_move.x != 0.0 || cursor_move.y != 0.0 {
						peripherals_update = Some(PeripheralsUpdate::MouseMove(cursor_move - mouse_update_fix));
						window.set_cursor_position(win::PhysicalPosition { x: window_center.x as f64, y: window_center.y as f64 }).unwrap();
					}
					mouse_update_fix = cursor_move;
				}
			},
			win::Event::WindowEvent { event: win::WindowEvent::Focused(new_focused), .. } => {
				focused = new_focused;
				window.set_cursor_visible(!focused);
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
