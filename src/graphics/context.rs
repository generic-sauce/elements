use crate::prelude::*;

pub struct GraphicsContext<'a> {
	pub device: &'a wgpu::Device,
	pub queue: &'a wgpu::Queue,
	pub swap_chain_texture: &'a wgpu::SwapChainTexture,
	pub encoder: &'a mut wgpu::CommandEncoder,
}
