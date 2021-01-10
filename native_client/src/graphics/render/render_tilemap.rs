use crate::graphics::*;

#[derive(Copy, Clone)]
struct Vertex {
	position: SurfaceVec,
	uv: TextureVec,
}

fn vertex_to_bytes_len() -> u64 {
	(3 + 2) * std::mem::size_of::<f32>() as u64
}

fn vertices_to_bytes(vertices: &[Vertex]) -> Vec<u8> {
	let vertices_size = vertices.len() * vertex_to_bytes_len() as usize;
	let mut bytes = Vec::<u8>::with_capacity(vertices_size);

	for vertex in vertices {
		bytes.extend(vertex.position.x.to_le_bytes().iter());
		bytes.extend(vertex.position.y.to_le_bytes().iter());
		bytes.extend((0.4 as f32).to_le_bytes().iter());
		bytes.extend(vertex.uv.x.to_le_bytes().iter());
		bytes.extend(vertex.uv.y.to_le_bytes().iter());
	}

	bytes
}

fn create_tilemap_texture(device: &wgpu::Device, tilemap_size: TileVec) -> (wgpu::Texture, wgpu::TextureView) {
	let tilemap_texture = device.create_texture(&wgpu::TextureDescriptor {
		label: Some("tilemap texture"),
		size: wgpu::Extent3d {
			width: tilemap_size.x as u32,
			height: tilemap_size.y as u32,
			depth: 1,
		},
		mip_level_count: 1,
		sample_count: 1,
		dimension: wgpu::TextureDimension::D2,
		format: wgpu::TextureFormat::R8Unorm,
		usage: wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED
	});

	let tilemap_texture_view = tilemap_texture.create_view(&wgpu::TextureViewDescriptor {
		label: Some("tilemap texture view"),
		..Default::default()
	});

	(tilemap_texture, tilemap_texture_view)
}

fn create_bind_group(device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout, tilemap_texture_view: &wgpu::TextureView, tilemap_sampler: &wgpu::Sampler) -> wgpu::BindGroup {
	let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
		label: Some("tilemap bind group"),
		layout: bind_group_layout,
		entries: &[
			wgpu::BindGroupEntry {
				binding: 0,
				resource: wgpu::BindingResource::TextureView(tilemap_texture_view),
			},
			wgpu::BindGroupEntry {
				binding: 1,
				resource: wgpu::BindingResource::Sampler(tilemap_sampler),
			},
		]
	});

	bind_group
}

pub(in crate::graphics) struct RenderTilemap {
	pipeline: wgpu::RenderPipeline,
	vertex_buffer: wgpu::Buffer,
	tilemap_size: TileVec,
	tilemap_texture: Option<wgpu::Texture>,
	tilemap_texture_view: Option<wgpu::TextureView>,
	tilemap_sampler: wgpu::Sampler,
	bind_group_layout: wgpu::BindGroupLayout,
	bind_group: Option<wgpu::BindGroup>,
}

impl RenderTilemap {
	fn create_vertex_buffer(device: &wgpu::Device, vertices_capacity: u64) -> wgpu::Buffer {
		let vertices_size = vertices_capacity * vertex_to_bytes_len();
		device.create_buffer(&wgpu::BufferDescriptor {
			label: Some("vertex buffer"),
			size: vertices_size,
			usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::VERTEX,
			mapped_at_creation: false
		})
	}

	pub(in crate::graphics) fn new(device: &wgpu::Device) -> RenderTilemap {
		let vertex_buffer = Self::create_vertex_buffer(device, 4);

		let vertex_buffer_desc = wgpu::VertexBufferDescriptor {
			stride: vertex_to_bytes_len(),
			step_mode: wgpu::InputStepMode::Vertex,
			attributes: &[
				wgpu::VertexAttributeDescriptor {
					offset: 0,
					format: wgpu::VertexFormat::Float3,
					shader_location: 0
				},
				wgpu::VertexAttributeDescriptor {
					offset: 3 * std::mem::size_of::<f32>() as u64,
					format: wgpu::VertexFormat::Float2,
					shader_location: 1
				},
			]
		};

		let vert = load_shader_from_file(device, res("shader/tilemap.vert.spv"));
		let frag = load_shader_from_file(device, res("shader/tilemap.frag.spv"));

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
				}
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
			depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
				format: wgpu::TextureFormat::Depth32Float,
				depth_write_enabled: true,
				depth_compare: wgpu::CompareFunction::Less,
				stencil: Default::default(),
			}),
			vertex_state: wgpu::VertexStateDescriptor {
				index_format: Default::default(),
				vertex_buffers: &[vertex_buffer_desc],
			},
			sample_count: 1,
			sample_mask: !0,
			alpha_to_coverage_enabled: false,
		});

		let tilemap_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
			label: Some("tilemap sampler"),
			..Default::default()
		});

		RenderTilemap {
			pipeline,
			vertex_buffer,
			tilemap_size: TileVec::new(0, 0),
			tilemap_texture: None,
			tilemap_texture_view: None,
			tilemap_sampler,
			bind_group_layout,
			bind_group: None,
		}
	}

	fn resize_tilemap(&mut self, device: &wgpu::Device, tilemap_size: TileVec) {
		if tilemap_size != self.tilemap_size {
			let (tilemap_texture, tilemap_texture_view) = create_tilemap_texture(device, tilemap_size);
			let bind_group = create_bind_group(device, &self.bind_group_layout, &tilemap_texture_view, &self.tilemap_sampler);

			self.tilemap_texture = Some(tilemap_texture);
			self.tilemap_texture_view = Some(tilemap_texture_view);
			self.bind_group = Some(bind_group);
			self.tilemap_size = tilemap_size;
		}
	}

	pub(in crate::graphics) fn render(
		&mut self,
		context: &mut GraphicsContext,
		draw: &RenderDraw,
	) {
		let tilemap = match &draw.tilemap {
			Some(tilemap) => tilemap,
			None => return,
		};

		assert!(tilemap.size != TileVec::new(0, 0));
		self.resize_tilemap(context.device, tilemap.size);

		let window_size = context.window_size.to_subpixel();
		let s = |v: ViewVec| v.to_surface(window_size);
		context.queue.write_buffer(&self.vertex_buffer, 0,
			&vertices_to_bytes(
				&[
					Vertex { position: s(v(0.0, 0.0)), uv: v(0.0, 0.0) },
					Vertex { position: s(v(1.0, 0.0)), uv: v(1.0, 0.0) },
					Vertex { position: s(v(0.0, 1.0)), uv: v(0.0, 1.0) },
					Vertex { position: s(v(1.0, 1.0)), uv: v(1.0, 1.0) },
				],
			)[..]
		);

		context.queue.write_texture(
			wgpu::TextureCopyView {
				texture: self.tilemap_texture.as_ref().unwrap(),
				mip_level: 0,
				origin: wgpu::Origin3d::ZERO,
			},
			&tilemap.data,
			wgpu::TextureDataLayout {
				offset: 0,
				bytes_per_row: self.tilemap_size.x as u32,
				rows_per_image: self.tilemap_size.y as u32,
			},
			wgpu::Extent3d {
				width: self.tilemap_size.x as u32,
				height: self.tilemap_size.y as u32,
				depth: 1,
			}
		);

		let color_load_op = context.color_load_op();
		let depth_load_op = context.depth_load_op();
		let mut render_pass = context.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			color_attachments: &[
				wgpu::RenderPassColorAttachmentDescriptor {
					attachment: &context.swap_chain_texture.view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: color_load_op,
						store: true
					}
				},
			],
			depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
				attachment: context.depth_texture_view,
				depth_ops: Some(wgpu::Operations {
					load: depth_load_op,
					store: true,
				}),
				stencil_ops: None,
			}),
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
