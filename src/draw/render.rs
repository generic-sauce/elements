use crate::prelude::*;

pub struct Renderer {
	pipeline: wgpu::RenderPipeline,
	bind_group: wgpu::BindGroup,
}

impl Renderer {
	pub fn new(device: &wgpu::Device) -> Renderer {
		// let texture_size = Vec2u::new(512, 512);
		// let texture_descriptor = wgpu::TextureDescriptor {
		// 	label: Some("render_texture"),
		// 	size: pixels::wgpu::Extent3d {
		// 		width: texture_size.x,
		// 		height: texture_size.y,
		// 		depth: 1,
		// 	},
		// 	mip_level_count: 1,
		// 	sample_count: 1,
		// 	dimension: wgpu::TextureDimension::D2,
		// 	format: wgpu::TextureFormat::Bgra8UnormSrgb,
		// 	usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::OUTPUT_ATTACHMENT,
		// };
		// let texture_view = device
		// 	.create_texture(&texture_descriptor)
		// 	.create_view(&wgpu::TextureViewDescriptor::default());
    //
		// let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
		// 	label: Some("render_sampler"),
		// 	address_mode_u: wgpu::AddressMode::ClampToEdge,
		// 	address_mode_v: wgpu::AddressMode::ClampToEdge,
		// 	address_mode_w: wgpu::AddressMode::ClampToEdge,
		// 	mag_filter: wgpu::FilterMode::Nearest,
		// 	min_filter: wgpu::FilterMode::Nearest,
		// 	mipmap_filter: wgpu::FilterMode::Nearest,
		// 	lod_min_clamp: 0.0,
		// 	lod_max_clamp: 1.0,
		// 	compare: None,
		// 	anisotropy_clamp: None,
		// });

		let vertex_shader = device.create_shader_module(wgpu::include_spirv!("../../res/shader/render.vert.spv"));
		let fragment_shader = device.create_shader_module(wgpu::include_spirv!("../../res/shader/render.frag.spv"));

		let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
			label: Some("render_bind_group_layout"),
			entries: &[],
		});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("render_bind_group"),
			layout: &bind_group_layout,
			entries: &[],
		});

		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("render_pipeline_layout"),
			bind_group_layouts: &[&bind_group_layout],
			push_constant_ranges: &[],
		});

		let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("render_pipeline"),
			layout: Some(&pipeline_layout),
			vertex_stage: wgpu::ProgrammableStageDescriptor {
				module: &vertex_shader,
				entry_point: "main"
			},
			fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
				module: &fragment_shader,
				entry_point: "main"
			}),
			rasterization_state: Some(wgpu::RasterizationStateDescriptor {
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: wgpu::CullMode::Back,
				clamp_depth: false,
				depth_bias: 0,
				depth_bias_slope_scale: 0.0,
				depth_bias_clamp: 0.0,
			}),
			primitive_topology: wgpu::PrimitiveTopology::TriangleList,
			color_states: &[wgpu::ColorStateDescriptor {
					format: wgpu::TextureFormat::Bgra8UnormSrgb,
					color_blend: wgpu::BlendDescriptor::REPLACE,
					alpha_blend: wgpu::BlendDescriptor::REPLACE,
					write_mask: wgpu::ColorWrite::ALL,
			}],
			depth_stencil_state: None,
			vertex_state: wgpu::VertexStateDescriptor {
					index_format: wgpu::IndexFormat::Uint16,
					vertex_buffers: &[],
			},
			sample_count: 1,
			sample_mask: !0,
			alpha_to_coverage_enabled: false,
		});

		Self {
			pipeline,
			bind_group,
		}
	}

	pub fn update(&self) {
		// TODO: update and bind buffers (uniforms, textures)
	}

	pub fn render(&self, encoder: &mut wgpu::CommandEncoder, render_target: &wgpu::TextureView) {
		let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
				attachment: render_target,
				resolve_target: None,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
					store: true,
				},
			}],
			depth_stencil_attachment: None,
		});
		rpass.set_pipeline(&self.pipeline);
		rpass.set_bind_group(0, &self.bind_group, &[]);
		rpass.draw(0..6, 0..1);
	}
}
