mod draw;
use draw::*;

mod context;
use context::*;

use crate::prelude::*;

const SURFACE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

pub struct Graphics {
	#[allow(unused)] instance: wgpu::Instance,
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	window_size: WindowVec,
	swap_chain: wgpu::SwapChain,
	triangles: DrawTriangles,
	tilemap: DrawTilemap,
	fluidmap: DrawFluidmap,
	text: DrawText,
}

fn create_swap_chain(device: &wgpu::Device, surface: &wgpu::Surface, window_size: WindowVec) -> wgpu::SwapChain {
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
			.unwrap();

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
		let window_size = WindowVec::new(window_size.width, window_size.height);

		let swap_chain = create_swap_chain(&device, &surface, window_size);

		let triangles = DrawTriangles::new(&device, &queue);
		let tilemap = DrawTilemap::new(&device);
		let fluidmap = DrawFluidmap::new(&device);
		let text = DrawText::new(&device);

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
		}
	}

	/* create and fill draw pass
	 * create and fill command buffer
	 * submit command buffer to queue
	 */
	pub fn render(&mut self, draw: &Draw) {
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

		let mut graphics_context = GraphicsContext {
			device: &self.device,
			queue: &self.queue,
			swap_chain_texture: &swap_chain_texture,
			encoder: &mut encoder,
			window_size: self.window_size,
		};

		let mut cleared = false;
		let mut clear_color = move || {
			let load_op = match cleared {
				true => wgpu::LoadOp::Load,
				false => wgpu::LoadOp::Clear(wgpu::Color {
					r: 50.0 / 255.0,
					g: 120.0 / 255.0,
					b: 215.0 / 255.0,
					a: 1.0,
				})
			};
			cleared = true;
			load_op
		};

		if let Some(world) = &draw.world {
			self.fluidmap.render(
				&mut graphics_context,
				clear_color(),
				world.tilemap_size,
				&world.fluidmap_data,
				draw.elapsed_time,
			);

			self.tilemap.render(
				&mut graphics_context,
				clear_color(),
				world.tilemap_size,
				&world.tilemap_data,
			);
		}

		self.triangles.render(
			&mut graphics_context,
			clear_color(),
			draw,
		);

		self.text.render(
			&mut graphics_context,
			draw,
		);

		self.queue.submit(Some(encoder.finish()));
	}

	pub fn resize(&mut self, size: WindowVec) {
		self.window_size = size;
		self.swap_chain = create_swap_chain(&self.device, &self.surface, size);
	}
}
