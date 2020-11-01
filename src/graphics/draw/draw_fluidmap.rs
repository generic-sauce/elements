use crate::prelude::*;
use crate::graphics::*;

#[derive(Copy, Clone)]
struct Vertex {
	position: SurfaceVec,
	uv: Vec2f,
}

fn vertex_to_bytes_len() -> u64 {
	2 * 2 * std::mem::size_of::<f32>() as u64
}

fn vertices_to_bytes(vertices: &[Vertex]) -> Vec<u8> {
	let vertices_size = vertices.len() * vertex_to_bytes_len() as usize;
	let mut bytes = Vec::<u8>::with_capacity(vertices_size);

	for vertex in vertices {
		bytes.extend(vertex.position.x.to_le_bytes().iter());
		bytes.extend(vertex.position.y.to_le_bytes().iter());
		bytes.extend(vertex.uv.x.to_le_bytes().iter());
		bytes.extend(vertex.uv.y.to_le_bytes().iter());
	}

	bytes
}

fn create_fluidmap_texture(device: &wgpu::Device, tilemap_size: TileVec) -> (wgpu::Texture, wgpu::TextureView) {
	let fluidmap_texture = device.create_texture(&wgpu::TextureDescriptor {
		label: Some("fluidmap texture"),
		size: wgpu::Extent3d {
			width: tilemap_size.x as u32,
			height: tilemap_size.y as u32,
			depth: 1,
		},
		mip_level_count: 1,
		sample_count: 1,
		dimension: wgpu::TextureDimension::D2,
		format: wgpu::TextureFormat::Rgba8Unorm,
		usage: wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED
	});

	let fluidmap_texture_view = fluidmap_texture.create_view(&wgpu::TextureViewDescriptor {
		label: Some("fluidmap texture view"),
		..Default::default()
	});

	(fluidmap_texture, fluidmap_texture_view)
}

fn create_bind_group(device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout, fluidmap_texture_view: &wgpu::TextureView, fluidmap_sampler: &wgpu::Sampler, uniform_buffer: &wgpu::Buffer) -> wgpu::BindGroup {
	let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
		label: Some("fluidmap bind group"),
		layout: bind_group_layout,
		entries: &[
			wgpu::BindGroupEntry {
				binding: 0,
				resource: wgpu::BindingResource::TextureView(fluidmap_texture_view),
			},
			wgpu::BindGroupEntry {
				binding: 1,
				resource: wgpu::BindingResource::Sampler(fluidmap_sampler),
			},
			wgpu::BindGroupEntry {
				binding: 2,
				resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
			},
		]
	});

	bind_group
}

fn create_uniform_buffer(device: &wgpu::Device) -> wgpu::Buffer {
	let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
		label: Some("uniform buffer"),
		size: 2 * std::mem::size_of::<f32>() as u64,
		usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
		mapped_at_creation: false
	});

	uniform_buffer
}

fn uniform_to_bytes(elapsed_time: f32) -> Vec<u8> {
	bytemuck::cast_slice(&[elapsed_time]).to_vec()
}

fn create_vertex_buffer(device: &wgpu::Device, vertices_capacity: u64) -> wgpu::Buffer {
	let vertices_size = vertices_capacity * vertex_to_bytes_len();
	let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
		label: Some("vertex buffer"),
		size: vertices_size,
		usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::VERTEX,
		mapped_at_creation: false
	});

	vertex_buffer
}

pub(in crate::graphics) struct DrawFluidmap {
	pipeline: wgpu::RenderPipeline,
	vertex_buffer: wgpu::Buffer,
	tilemap_size: TileVec,
	fluidmap_texture: Option<wgpu::Texture>,
	fluidmap_texture_view: Option<wgpu::TextureView>,
	fluidmap_sampler: wgpu::Sampler,
	uniform_buffer: wgpu::Buffer,
	bind_group_layout: wgpu::BindGroupLayout,
	bind_group: Option<wgpu::BindGroup>,
}

impl DrawFluidmap {
	pub(in crate::graphics) fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> DrawFluidmap {
		let vertex_buffer = create_vertex_buffer(device, 4);
		queue.write_buffer(&vertex_buffer, 0, &vertices_to_bytes(&vec!(
			Vertex { position: v(-1.0, -1.0), uv: v(0.0, 0.0) },
			Vertex { position: v( 1.0, -1.0), uv: v(1.0, 0.0) },
			Vertex { position: v(-1.0,  1.0), uv: v(0.0, 1.0) },
			Vertex { position: v( 1.0,  1.0), uv: v(1.0, 1.0) },
		))[..]);

		let vertex_buffer_desc = wgpu::VertexBufferDescriptor {
			stride: vertex_to_bytes_len(),
			step_mode: wgpu::InputStepMode::Vertex,
			attributes: &[
				wgpu::VertexAttributeDescriptor {
					offset: 0,
					format: wgpu::VertexFormat::Float2,
					shader_location: 0
				},
				wgpu::VertexAttributeDescriptor {
					offset: 2 * std::mem::size_of::<f32>() as u64,
					format: wgpu::VertexFormat::Float2,
					shader_location: 1
				},
			]
		};

		let vert = device.create_shader_module(wgpu::include_spirv!("../../../res/shader/fluidmap.vert.spv"));
		let frag = device.create_shader_module(wgpu::include_spirv!("../../../res/shader/fluidmap.frag.spv"));

		let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
			label: Some("bind group layout"),
			entries: &[
				wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStage::FRAGMENT,
					count: None,
					ty: wgpu::BindingType::SampledTexture {
						dimension: wgpu::TextureViewDimension::D2,
						component_type: wgpu::TextureComponentType::Float,
						multisampled: false
					},
				},
				wgpu::BindGroupLayoutEntry {
					binding: 1,
					visibility: wgpu::ShaderStage::FRAGMENT,
					count: None,
					ty: wgpu::BindingType::Sampler {
						comparison: false
					},
				},
				wgpu::BindGroupLayoutEntry {
					binding: 2,
					visibility: wgpu::ShaderStage::FRAGMENT,
					count: None,
					ty: wgpu::BindingType::UniformBuffer {
						dynamic: false,
						min_binding_size: None
					},
				},
			]
		});

		let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
			label: Some("pipeline layout descriptor"),
			bind_group_layouts: &[
				&bind_group_layout,
			],
			push_constant_ranges: &[]
		});

		let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("Render pipeline"),
			layout: Some(&pipeline_layout),
			vertex_stage: wgpu::ProgrammableStageDescriptor {
					module: &vert,
					entry_point: "main",
			},
			fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
					module: &frag,
					entry_point: "main",
			}),
			rasterization_state: Some(wgpu::RasterizationStateDescriptor {
					front_face: wgpu::FrontFace::Ccw,
					cull_mode: wgpu::CullMode::None,
					..Default::default()
			}),
			primitive_topology: wgpu::PrimitiveTopology::TriangleStrip,
			color_states: &[SURFACE_FORMAT.into()],
			depth_stencil_state: None,
			vertex_state: wgpu::VertexStateDescriptor {
				index_format: Default::default(),
				vertex_buffers: &[vertex_buffer_desc],
			},
			sample_count: 1,
			sample_mask: !0,
			alpha_to_coverage_enabled: false,
		});

		let fluidmap_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			label: Some("fluidmap sampler"),
			..Default::default()
		});

		let uniform_buffer = create_uniform_buffer(device);

		DrawFluidmap {
			pipeline,
			vertex_buffer,
			tilemap_size: TileVec::new(0, 0),
			fluidmap_texture: None,
			fluidmap_texture_view: None,
			fluidmap_sampler,
			uniform_buffer,
			bind_group_layout,
			bind_group: None,
		}
	}

	fn resize_fluidmap(&mut self, device: &wgpu::Device, tilemap_size: TileVec) {
		if tilemap_size != self.tilemap_size {
			let (fluidmap_texture, fluidmap_texture_view) = create_fluidmap_texture(device, tilemap_size);

			self.fluidmap_texture = Some(fluidmap_texture);
			self.fluidmap_texture_view = Some(fluidmap_texture_view);
			self.tilemap_size = tilemap_size;
		}
	}

	pub(in crate::graphics) fn render(
		&mut self,
		context: &mut GraphicsContext,
		load: wgpu::LoadOp::<wgpu::Color>,
		world: &GraphicsWorld,
	) {
		assert!(world.tilemap_size != TileVec::new(0, 0));
		self.resize_fluidmap(context.device, world.tilemap_size);

		context.queue.write_texture(
			wgpu::TextureCopyView {
				texture: self.fluidmap_texture.as_ref().unwrap(),
				mip_level: 0,
				origin: wgpu::Origin3d::ZERO,
			},
			&world.fluidmap_data,
			wgpu::TextureDataLayout {
				offset: 0,
				bytes_per_row: 4 * world.tilemap_size.x as u32,
				rows_per_image: world.tilemap_size.y as u32,
			},
			wgpu::Extent3d {
				width: world.tilemap_size.x as u32,
				height: world.tilemap_size.y as u32,
				depth: 1,
			}
		);

		let elapsed_time = world.elapsed_time.as_millis() as f32 / 1009.0;
		context.queue.write_buffer(&self.uniform_buffer, 0, &uniform_to_bytes(elapsed_time)[..]);
		self.bind_group = Some(create_bind_group(
			context.device,
			&self.bind_group_layout,
			&self.fluidmap_texture_view.as_ref().unwrap(),
			&self.fluidmap_sampler,
			&self.uniform_buffer
		));

		let mut render_pass = context.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			color_attachments: &[
				wgpu::RenderPassColorAttachmentDescriptor {
					attachment: &context.swap_chain_texture.view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: load,
						store: true
					}
				},
			],
			depth_stencil_attachment: None
		});

		render_pass.set_pipeline(&self.pipeline);
		render_pass.set_bind_group(
			0,
			self.bind_group.as_ref().unwrap(),
			&[]
		);
		render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
		render_pass.draw(0 .. 4, 0 .. 1);
	}
}
