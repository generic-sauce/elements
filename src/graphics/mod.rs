pub mod world;
mod draw_triangles;

use crate::prelude::*;
use draw_triangles::*;

pub const SURFACE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

pub struct Graphics {
	instance: wgpu::Instance,
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	window_size: Vec2u,
	swap_chain: wgpu::SwapChain,
	pub triangles: DrawTriangles,
}

fn create_swap_chain(device: &wgpu::Device, surface: &wgpu::Surface, size: Vec2u) -> wgpu::SwapChain {
	let mut swap_chain_desc = wgpu::SwapChainDescriptor {
		usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
		format: SURFACE_FORMAT,
		width: size.x,
		height: size.y,
		present_mode: wgpu::PresentMode::Fifo,
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
		let window_size = Vec2u::new(window_size.width, window_size.height);

		let swap_chain = create_swap_chain(&device, &surface, window_size);

		let triangles = DrawTriangles::new(&device);

		Graphics {
			instance,
			surface,
			device,
			queue,
			window_size,
			swap_chain,
			triangles,
		}
	}

	pub fn draw(&mut self) {
		// self.draw_hud(...);
		self.draw_players();
		// self.draw_tilemap(...);
		// self.draw_fluids(...);
		// self.draw_background(...);
	}

	fn draw_players(&mut self) {
		self.triangles.draw_sprite(CanvasVec::new(0.0, 0.0), CanvasVec::new(0.5, 0.5), Some(wgpu::Color::RED));
		self.triangles.draw_sprite(CanvasVec::new(-0.3, -0.3), CanvasVec::new(0.5, 0.5), Some(wgpu::Color::GREEN));
		self.triangles.draw_sprite(CanvasVec::new(0.1, -0.4), CanvasVec::new(0.5, 0.5), Some(wgpu::Color::BLUE));
	}

	/* create and fill draw pass
	 * create and fill command buffer
	 * submit command buffer to queue
	 */
	pub fn flush(&mut self) {
		let swap_chain_texture = self.swap_chain
			.get_current_frame()
			.unwrap()
			.output;

		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("command encoder")
		});

		let clear_color = wgpu::Color {
			r: 0.03,
			g: 0.025,
			b: 0.025,
			a: 1.0,
		};

		self.triangles.flush(
			&self.device,
			&self.queue,
			&mut encoder,
			&swap_chain_texture,
			wgpu::LoadOp::Clear(clear_color)
		);

		self.queue.submit(Some(encoder.finish()));
	}

	pub fn resize(&mut self, size: Vec2u) {
		self.window_size = size;
		self.swap_chain = create_swap_chain(&self.device, &self.surface, size);
	}
}
