#![feature(drain_filter)]
#![feature(const_fn)]
#![feature(generic_associated_types)]

include!("base.rs");

use crate::prelude::*;

const DEFAULT_CURSOR_POSITION: WindowVec = WindowVec::new(300.0, 300.0);

#[cfg(feature = "native-client")]
fn main() {
	let server_arg = std::env::args().nth(1);
	if let Some("server") = server_arg.as_deref() {
		Server::new().run();
		return;
	}

	let (graphics_sender, graphics_receiver) = channel::<Draw>();
	let (input_sender, input_receiver) = channel::<PeripheralsUpdate>();

	thread::spawn(move || {
		let input_backend = NativeInputBackend::new(input_receiver);
		match server_arg.as_deref() {
			Some("menu") => App::<NativeBackend>::new(graphics_sender, input_backend).run_menu_and_game(),
			Some(ip) => App::<NativeBackend>::new(graphics_sender, input_backend).run_client(ip),
			None => App::<NativeBackend>::new(graphics_sender, input_backend).run_local(0),
		}
	});

	let event_loop = win::EventLoop::new();
	let window = win::WindowBuilder::new()
		.with_inner_size(win::PhysicalSize::new(1280, 720))
		// .with_resizable(false)
		.with_title("Elements")
		.build(&event_loop)
		.unwrap();

	let mut graphics = Graphics::new(&window);
	let mut graphics_world = graphics_receiver.recv().unwrap();

	event_loop.run(move |event, _window_target, control_flow| {
		*control_flow = win::ControlFlow::Poll;

		let mut peripherals_update: Option<PeripheralsUpdate> = None;

		match event {
			win::Event::WindowEvent { event: win::WindowEvent::CloseRequested, .. } => {
				*control_flow = win::ControlFlow::Exit;
			},
			win::Event::WindowEvent { event: win::WindowEvent::CursorMoved { position, .. }, .. } => {
				let cursor_position = WindowVec::new(position.x as f32, position.y as f32);
				let cursor_move = cursor_position - DEFAULT_CURSOR_POSITION;
				if cursor_move.x != 0.0 || cursor_move.y != 0.0 {
					peripherals_update = Some(PeripheralsUpdate::MouseMove(cursor_move));
					window.set_cursor_position(win::PhysicalPosition { x: DEFAULT_CURSOR_POSITION.x as f64, y: DEFAULT_CURSOR_POSITION.y as f64 }).unwrap();
				}
			},
			win::Event::WindowEvent { event: win::WindowEvent::Resized(size), .. } => {
				graphics.resize(Vec2u::new(size.width, size.height));
			},
			win::Event::WindowEvent { event: win::WindowEvent::ReceivedCharacter(c), .. } => {
				peripherals_update = Some(PeripheralsUpdate::Text(c));
			},
			win::Event::WindowEvent { event: win::WindowEvent::KeyboardInput { input: win::KeyboardInput { virtual_keycode: Some(virtual_keycode), state, .. }, .. }, .. } => {
				match state {
					win::ElementState::Pressed => {
						peripherals_update = Some(PeripheralsUpdate::KeyPress(Key::from(virtual_keycode)));
					}
					win::ElementState::Released => {
						peripherals_update = Some(PeripheralsUpdate::KeyRelease(Key::from(virtual_keycode)));
					}
				}
			},
			win::Event::WindowEvent { event: win::WindowEvent::MouseInput { state, button, .. }, .. } => {
				match state {
					win::ElementState::Pressed => {
						peripherals_update = Some(PeripheralsUpdate::KeyPress(Key::from(button)));
					}
					win::ElementState::Released => {
						peripherals_update = Some(PeripheralsUpdate::KeyRelease(Key::from(button)));
					}
				}
			},
			win::Event::MainEventsCleared => {
				window.request_redraw();
			},
			win::Event::RedrawRequested {..} => {
				if let Ok(world) = graphics_receiver.try_recv() { graphics_world = world };
				let window_size = window.inner_size();
				let window_size = Vec2u::new(window_size.width, window_size.height);
				let mut draw = Draw::new(window_size);
				/*
				graphics.draw(&mut draw, &graphics_world);
				graphics.flush(&draw, &graphics_world);
				 */
			},
			_ => ()
		}

		if let Some(update) = peripherals_update {
			input_sender.send(update).unwrap();
		}

		window.set_cursor_grab(true).unwrap();
		window.set_cursor_visible(false);
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
