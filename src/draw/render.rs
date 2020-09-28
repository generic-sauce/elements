use crate::prelude::*;

pub struct Renderer {
	// bind_group: wgpu::BindGroup,
	// render_pipeline: wgpu::RenderPipeline,
	// time_buffer: wgpu::Buffer,
}

impl Renderer {
	pub fn new(device: &wgpu::Device) -> Renderer {
		let vertex_shader = device.create_shader_module(wgpu::include_spirv!("../../res/shader/render.vert.spv"));
		let fragment_shader = device.create_shader_module(wgpu::include_spirv!("../../res/shader/render.frag.spv"));

		let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
			label: Some("render_bind_group_layout_descriptor"),
			entries: &[
			],
		});

		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("render_pipeline_layout"),
			bind_group_layouts: &[&bind_group_layout],
			push_constant_ranges: &[],
		});

		Renderer {}
	}

	pub fn update(&self) {
		// TODO: update and bind buffers (uniforms, textures)
	}

	pub fn render(&self) {
		// TODO: run renderpass
	}
}
