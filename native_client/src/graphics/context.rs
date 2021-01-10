use crate::graphics::*;

pub(in crate::graphics) struct GraphicsContext<'a> {
	pub device: &'a wgpu::Device,
	pub queue: &'a wgpu::Queue,
	pub swap_chain_texture: &'a wgpu::SwapChainTexture,
	pub encoder: &'a mut wgpu::CommandEncoder,
	pub window_size: PixelVec,
	pub clear_color: wgpu::Color,
	color_cleared: bool,
	pub elapsed_ms: f32,
}

impl<'a> GraphicsContext<'a> {
	pub(in crate::graphics) fn new(
		device: &'a wgpu::Device,
		queue: &'a wgpu::Queue,
		swap_chain_texture: &'a wgpu::SwapChainTexture,
		encoder: &'a mut wgpu::CommandEncoder,
		window_size: PixelVec,
		clear_color: wgpu::Color,
		elapsed_ms: f32,
	) -> GraphicsContext<'a> {
		GraphicsContext {
			device,
			queue,
			swap_chain_texture,
			encoder,
			window_size,
			clear_color,
			color_cleared: false,
			elapsed_ms,
		}
	}

	pub(in crate::graphics) fn color_load_op(&mut self) -> wgpu::LoadOp<wgpu::Color> {
		let load_op = match self.color_cleared {
			true => wgpu::LoadOp::Load,
			false => wgpu::LoadOp::Clear(self.clear_color),
		};
		self.color_cleared = true;
		load_op
	}
}
