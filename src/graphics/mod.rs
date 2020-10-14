mod render_triangles;

use crate::prelude::*;
use render_triangles::*;

pub const SURFACE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

pub struct Graphics {
	instance: wgpu::Instance,
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	window_size: Vec2u,
	swap_chain: wgpu::SwapChain,
	render_triangles: RenderTriangles,
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

		let mut swap_chain_desc = wgpu::SwapChainDescriptor {
			usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
			format: SURFACE_FORMAT,
			width: window_size.x,
			height: window_size.y,
			present_mode: wgpu::PresentMode::Fifo,
		};

		let mut swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

		let render_triangles = RenderTriangles::new(&device);

		Graphics {
			instance,
			surface,
			device,
			queue,
			window_size,
			swap_chain,
			render_triangles,
		}
	}

	/* create and fill render pass
	 * create and fill command buffer
	 * submit command buffer to queue
	 */
	pub fn render(&mut self) {
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

		self.render_triangles.draw_rectangle(v(0.0, 0.0), CanvasVec::new(0.5, 0.5), Some(wgpu::Color::RED));

		self.render_triangles.flush(
			&self.device,
			&self.queue,
			&mut encoder,
			&swap_chain_texture,
			wgpu::LoadOp::Clear(clear_color)
		);

		self.queue.submit(Some(encoder.finish()));
	}

	/* recreate swap chain
	 */
	pub fn resize(&mut self, size: Vec2u) {
		let mut swap_chain_desc = wgpu::SwapChainDescriptor {
			usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
			format: SURFACE_FORMAT,
			width: size.x,
			height: size.y,
			present_mode: wgpu::PresentMode::Fifo,
		};

		self.window_size = size;
		self.swap_chain = self.device.create_swap_chain(&self.surface, &swap_chain_desc);
	}
}
