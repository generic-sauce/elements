#![feature(drain_filter)]
#![feature(const_fn)]
#![allow(incomplete_features)]
#![feature(generic_associated_types)]

include!("base.rs");

use crate::prelude::*;

const DEFAULT_CURSOR_POSITION: SubPixelVec = SubPixelVec::new(300.0, 300.0);

#[cfg(feature = "native-client")]
fn main() {
	let server_arg = std::env::args().nth(1);
	if let Some("server") = server_arg.as_deref() {
		Server::new().run();
		return;
	}

	let (draw_sender, draw_receiver) = channel::<Draw>();
	let (peripherals_sender, peripherals_receiver) = channel::<PeripheralsUpdate>();

	thread::spawn(move || {
		let mut runnable = match server_arg.as_deref() {
			Some("menu") => Runnable::Menu,
			Some(ip) => Runnable::Client(Client::new(ip)),
			None => Runnable::Local(Local::new(0)),
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

#[cfg(not(feature = "client"))]
fn main() {
	Server::new().run();
}
