#![feature(drain_filter)]
#![feature(const_fn)]

include!("base.rs");

use crate::prelude::*;

#[cfg(feature = "native-client")]
fn main() {
	let server_arg = std::env::args().nth(1);
	if let Some("server") = server_arg.as_deref() {
		Server::new().run();
		return;
	}

	let (graphics_sender, graphics_receiver) = channel::<GraphicsWorld>();
	let (input_sender, input_receiver) = channel::<KeyboardUpdate>();

	thread::spawn(move || {
		match server_arg.as_deref() {
			Some("menu") => App::new(graphics_sender, input_receiver).run_menu_and_game(),
			Some(ip) => App::new(graphics_sender, input_receiver).run_client(ip),
			None => App::new(graphics_sender, input_receiver).run_local(0),
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
	let mut frames = 0;

	event_loop.run(move |event, _window_target, control_flow| {
		*control_flow = win::ControlFlow::Poll;

		let mut keyboard_update: Option<KeyboardUpdate> = None;

		match event {
			win::Event::WindowEvent { event: win::WindowEvent::CloseRequested, .. } => {
				*control_flow = win::ControlFlow::Exit;
			},
			win::Event::WindowEvent { event: win::WindowEvent::Resized(size), .. } => {
				graphics.resize(Vec2u::new(size.width, size.height));
			},
			win::Event::WindowEvent { event: win::WindowEvent::ReceivedCharacter(c), .. } => {
				keyboard_update = Some(KeyboardUpdate::Text(c));
			},
			win::Event::WindowEvent { event: win::WindowEvent::KeyboardInput { input: win::KeyboardInput { virtual_keycode: Some(virtual_keycode), state, .. }, .. }, .. } => {
				match state {
					win::ElementState::Pressed => {
						keyboard_update = Some(KeyboardUpdate::KeyPress(Key::from(virtual_keycode)));
					}
					win::ElementState::Released => {
						keyboard_update = Some(KeyboardUpdate::KeyRelease(Key::from(virtual_keycode)));
					}
				}
			},
			win::Event::MainEventsCleared => {
				window.request_redraw();
			},
			win::Event::RedrawRequested {..} => {
				if let Ok(world) = graphics_receiver.try_recv() { graphics_world = world };
				graphics.draw(&graphics_world);
				graphics.flush(&graphics_world);
				frames += 1;
				if frames % 1000 == 0 {
					println!("{} fps on wgpu", frames / (graphics_world.elapsed_time.as_secs() + 1));
				}
			},
			_ => ()
		}

		if let Some(keyboard_update) = keyboard_update {
			input_sender.send(keyboard_update);
		}

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
