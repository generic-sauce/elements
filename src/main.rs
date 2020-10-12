#![feature(drain_filter)]
#![feature(const_fn)]

#[macro_use]
extern crate serde_derive;

#[cfg(feature = "client")]
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "client")] mod animation_state;
#[cfg(feature = "client")] mod client;
#[cfg(feature = "client")] mod texture_state;
#[cfg(feature = "client")] mod shader_state;
#[cfg(feature = "client")] mod font_state;
#[cfg(feature = "client")] mod draw_context;
#[cfg(feature = "client")] mod app;
#[cfg(feature = "client")] mod local;
#[cfg(feature = "client")] mod draw;
#[cfg(feature = "client")] mod input;
#[cfg(feature = "client")] mod window_vec;
#[cfg(feature = "client")] mod menu;
#[cfg(feature = "client")] mod graphics;

#[macro_use]
mod fps_timer;

mod timed_loop;
mod world;
mod vec;
mod server;
mod net;
mod animation;
mod resource;
mod prelude;

use crate::prelude::*;

#[cfg(feature = "client")]
fn main() {
	let arg = std::env::args().nth(1);

	if let Some("server") = arg.as_deref() {
		Server::new().run();
		return;
	}

	// main program
	thread::spawn(move || {
		match arg.as_deref() {
			Some(ip) => App::new().run_client(ip),
			None => App::new().run_local(),
		}
	});

	let event_loop = win::EventLoop::new();
	let window = win::WindowBuilder::new()
		.with_inner_size(win::PhysicalSize::new(1280, 720))
		.with_resizable(false)
		.with_title("Elements")
		.build(&event_loop)
		.unwrap();

	let (window, mut graphics) = Graphics::new(window);

	event_loop.run(move |event, window_target, control_flow| {
		*control_flow = win::ControlFlow::Poll;

		match event {
			win::Event::WindowEvent {event: win::WindowEvent::CloseRequested, ..} => {
				*control_flow = win::ControlFlow::Exit;
			},
			win::Event::WindowEvent {event: win::WindowEvent::Resized(size), ..} => {
				graphics.resize(Vec2u::new(size.width, size.height));
				// swap_chain_desc.width = size.width;
				// swap_chain_desc.height = size.height;
				// swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);
			},
			win::Event::MainEventsCleared => {
				window.request_redraw();
			},
			win::Event::RedrawRequested {..} => {
				graphics.render();
				// let frame = swap_chain
				// 	.get_current_frame()
				// 	.unwrap()
				// 	.output;

				// let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
				// 	label: Some("command encoder")
				// });
        //
				// let dur = start_time.elapsed().as_millis() as f32 / 1000.0;
				// let uniforms = vec!(
				// 	(dur + std::f32::consts::PI * 0.0/3.0).sin() * 0.5 + 0.5,
				// 	(dur + std::f32::consts::PI * 1.0/3.0).sin() * 0.5 + 0.5,
				// 	(dur + std::f32::consts::PI * 2.0/3.0).sin() * 0.5 + 0.5,
				// );
				// queue.write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&uniforms));
        //
				// {
				// 	let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				// 		color_attachments: &[
				// 			wgpu::RenderPassColorAttachmentDescriptor {
				// 				attachment: &frame.view,
				// 				resolve_target: None,
				// 				ops: wgpu::Operations {
				// 					load: wgpu::LoadOp::Clear(wgpu::Color {
				// 						r: 0.03,
				// 						g: 0.025,
				// 						b: 0.025,
				// 						a: 1.0,
				// 					}),
				// 					store: true
				// 				}
				// 			},
				// 		],
				// 		depth_stencil_attachment: None
				// 	});
        //
				// 	render_pass.set_pipeline(&render_pipeline);
				// 	render_pass.set_bind_group(
				// 		0,
				// 		&bind_group,
				// 		&[]
				// 	);
				// 	render_pass.set_vertex_buffer(0, vert_buffer.slice(..));
				// 	render_pass.draw(0..3, 0..1);
				// }
        //
				// queue.submit(Some(encoder.finish()));
			},
			_ => ()
		}
	});
}

#[cfg(not(feature = "client"))]
fn main() {
	Server::new().run();
}
