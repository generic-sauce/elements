use crate::prelude::*;

pub struct Renderer {
	pipeline: wgpu::RenderPipeline,
	bind_group: wgpu::BindGroup,
	texture_size: Vec2u,
	texture: wgpu::Texture,
}

impl Renderer {
	pub fn new(device: &wgpu::Device) -> Renderer {
		let texture_size = Vec2u::new(256, 256);
		let texture_descriptor = wgpu::TextureDescriptor {
			label: Some("render_texture"),
			size: pixels::wgpu::Extent3d {
				width: texture_size.x,
				height: texture_size.y,
				depth: 1,
			},
			mip_level_count: 1,
			sample_count: 1,
			dimension: wgpu::TextureDimension::D2,
			format: wgpu::TextureFormat::R8Unorm,
			usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST
		};

		let texture = device.create_texture(&texture_descriptor);
		let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

		let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			label: Some("render_sampler"),
			address_mode_u: wgpu::AddressMode::ClampToEdge,
			address_mode_v: wgpu::AddressMode::ClampToEdge,
			address_mode_w: wgpu::AddressMode::ClampToEdge,
			mag_filter: wgpu::FilterMode::Nearest,
			min_filter: wgpu::FilterMode::Nearest,
			mipmap_filter: wgpu::FilterMode::Nearest,
			lod_min_clamp: 0.0,
			lod_max_clamp: 1.0,
			compare: None,
			anisotropy_clamp: None,
		});

		let vertex_shader = device.create_shader_module(wgpu::include_spirv!("../../res/shader/render.vert.spv"));
		let fragment_shader = device.create_shader_module(wgpu::include_spirv!("../../res/shader/render.frag.spv"));

		let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
			label: Some("render_bind_group_layout"),
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStage::FRAGMENT,
					ty: wgpu::BindingType::SampledTexture {
						dimension: wgpu::TextureViewDimension::D2,
						component_type: wgpu::TextureComponentType::Float,
						multisampled: false,
					},
					count: None,
				},
				wgpu::BindGroupLayoutEntry {
					binding: 1,
					visibility: wgpu::ShaderStage::FRAGMENT,
					ty: wgpu::BindingType::Sampler {
						comparison: false,
					},
					count: None,
				},
			],
		});

		let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			label: Some("render_bind_group"),
			layout: &bind_group_layout,
			entries: &[
				wgpu::BindGroupEntry {
					binding: 0,
					resource: wgpu::BindingResource::TextureView(&texture_view),
				},
				wgpu::BindGroupEntry {
					binding: 1,
					resource: wgpu::BindingResource::Sampler(&sampler),
				},
			],
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
			texture_size,
			texture,
		}
	}

	pub fn update(&self, queue: &wgpu::Queue) {
		let size = self.texture_size.x * self.texture_size.y;
		let mut data = Vec::with_capacity(size as usize);
		for i in 0..size {
			data.push(rand::random::<u8>());
		}

		queue.write_texture(
			wgpu::TextureCopyViewBase {
				texture: &self.texture,
				mip_level: 0,
				origin: wgpu::Origin3d {
					x: 0,
					y: 0,
					z: 0,
				}
			},
			data.as_slice(),
			wgpu::TextureDataLayout {
				offset: 0,
				bytes_per_row: self.texture_size.x,
				rows_per_image: 0,
			},
			wgpu::Extent3d {
				width: self.texture_size.x,
				height: self.texture_size.y,
				depth: 1,
			}
		);
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
