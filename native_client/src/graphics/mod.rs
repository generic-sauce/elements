use crate::prelude::*;

mod render;
use render::*;

mod context;
use context::*;

mod misc;
use misc::*;

const SURFACE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

pub fn color_to_wgpu(color: Color) -> wgpu::Color {
	wgpu::Color {
		r: color.r as f64,
		g: color.g as f64,
		b: color.b as f64,
		a: color.a as f64,
	}
}

pub struct Graphics {
	#[allow(unused)] instance: wgpu::Instance,
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	window_size: PixelVec,
	swap_chain: wgpu::SwapChain,
	triangles: RenderTriangles,
	tilemap: RenderTilemap,
	fluidmap: RenderFluidmap,
	text: RenderText,
	timer: Timer,
}

fn create_swap_chain(device: &wgpu::Device, surface: &wgpu::Surface, window_size: PixelVec) -> wgpu::SwapChain {
	let swap_chain_desc = wgpu::SwapChainDescriptor {
		usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
		format: SURFACE_FORMAT,
		width: window_size.x,
		height: window_size.y,
		present_mode: wgpu::PresentMode::Immediate,
	};

	device.create_swap_chain(surface, &swap_chain_desc)
}

impl Graphics {
	/* create instance
	 * create surface
	 * choose adapter (physical device)
	 * get device
	 */
	pub fn new(window: &win::Window) -> Graphics {
		let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

		let surface = unsafe { instance.create_surface(window) };
		let adapter = futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
			power_preference: wgpu::PowerPreference::HighPerformance,
			compatible_surface: Some(&surface)
		}))
			.expect("Failed to connect to graphics backend. Make sure \"vulkan\", \"DirectX\" or \"Metal\" is correctly installed!");

		let (device, queue) = futures::executor::block_on(adapter.request_device(
			&wgpu::DeviceDescriptor {
				features: wgpu::Features::empty(),
				limits: Default::default(),
				shader_validation: true
			},
			None
		))
			.unwrap();

		let window_size = window.inner_size();
		let window_size = PixelVec::new(window_size.width, window_size.height);

		let swap_chain = create_swap_chain(&device, &surface, window_size);

		let triangles = RenderTriangles::new(&device, &queue);
		let tilemap = RenderTilemap::new(&device);
		let fluidmap = RenderFluidmap::new(&device);
		let text = RenderText::new(&device);

		let timer = Timer::new();

		Graphics {
			instance,
			surface,
			device,
			queue,
			window_size,
			swap_chain,
			triangles,
			tilemap,
			fluidmap,
			text,
			timer,
		}
	}

	/* create and fill render pass
	 * create and fill command buffer
	 * submit command buffer to queue
	 */
	pub fn render(&mut self, draw: Draw) {
		let swap_chain_texture = match self.swap_chain.get_current_frame() {
			Ok(frame) => frame.output,
			Err(err) => {
				println!("swap chain error: {}. recreate and try again...", match err {
					wgpu::SwapChainError::Timeout => "timeout",
					wgpu::SwapChainError::Outdated => "outdated",
					wgpu::SwapChainError::Lost => "lost",
					wgpu::SwapChainError::OutOfMemory => "out of memory",
				});

				self.swap_chain = create_swap_chain(&self.device, &self.surface, self.window_size);
				self.swap_chain.get_current_frame().unwrap().output
			},
		};

		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("command encoder")
		});

		let draw = RenderDraw::new(draw, self.window_size.to_subpixel());
		let clear_color = color_to_wgpu(Color::rgba(
			f32::powf(draw.clear_color.r, 2.2),
			f32::powf(draw.clear_color.g, 2.2),
			f32::powf(draw.clear_color.b, 2.2),
			draw.clear_color.a,
		));

		let mut context = GraphicsContext::new(
			&self.device,
			&self.queue,
			&swap_chain_texture,
			&mut encoder,
			self.window_size,
			clear_color,
			self.timer.elapsed_ms(),
		);

		let mut indices = [0 as usize; DRAW_COMMAND_COUNT];
		let mut vertex_count = 0;

		self.triangles.set_vertices(&mut context, &draw);

		for command in &draw.commands {
			let index = &mut indices[*command as usize];
			match command {
				DrawCommand::Text => {
					self.text.render(
						&mut context,
						&draw.texts[*index],
					);
				},
				DrawCommand::Tilemap => {
					self.tilemap.render(
						&mut context,
						&draw,
					);
				},
				DrawCommand::Fluidmap => {
					self.fluidmap.render(
						&mut context,
						&draw,
					);
				},
				DrawCommand::Triangles => {
					let count = draw.triangle_commands[*index].count;
					self.triangles.render(
						&mut context,
						draw.triangle_commands[*index].texture_index,
						vertex_count,
						vertex_count + count,
					);
					vertex_count += count;
				},
			}

			*index += 1;
		}

		// self.triangles.render(
		// 	&mut graphics_context,
		// 	&draw,
		// );
    //
		// self.tilemap.render(
		// 	&mut graphics_context,
		// 	&draw,
		// );
    //
		// self.fluidmap.render(
		// 	&mut graphics_context,
		// 	&draw,
		// );
    //
		// self.text.render(
		// 	&mut graphics_context,
		// 	&draw,
		// );

		self.queue.submit(Some(encoder.finish()));
	}

	pub fn resize(&mut self, size: PixelVec) {
		self.window_size = size;
		self.swap_chain = create_swap_chain(&self.device, &self.surface, size);
	}
}
