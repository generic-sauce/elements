use super::*;

pub struct RenderDrawTilemap {
	pub size: TileVec,
	pub data: Vec<u8>,
	pub depth_value: DepthValue,
}

pub struct RenderDrawFluidmap {
	pub size: FluidVec,
	pub data: Vec<u8>,
	pub depth_value: DepthValue,
}

pub struct RenderDraw {
	pub clear_color: Color,
	pub vertex_data: Vec<u8>, // vertices for all textures in bytes
	pub vertex_counts: Vec<u32>, // number of vertices per texture
	pub texts: Vec<Text>,
	pub tilemap: Option<RenderDrawTilemap>,
	pub fluidmap: Option<RenderDrawFluidmap>,
}

impl RenderDraw {
	pub fn new(draw: Draw, window_size: SubPixelVec) -> RenderDraw {
		let Draw { clear_color, depth_index, texture_triangles, texts, tilemap, fluidmap } = draw;

		let clear_color = clear_color.unwrap_or(Color::BLACK);
		let clear_color = Color::rgb(
			f32::powf(clear_color.r, 2.2),
			f32::powf(clear_color.g, 2.2),
			f32::powf(clear_color.b, 2.2),
		);

		let mut vertex_counts = Vec::new();
		let mut vertex_data = Vec::new();

		for triangles in texture_triangles {
			let count = 3 * triangles.len() as u32;
			let bytes = triangles_to_bytes(window_size, &triangles[..], depth_index);

			vertex_counts.push(count);
			vertex_data.extend(bytes);
		}

		let tilemap = tilemap.map(|tilemap|
			RenderDrawTilemap {
				size: tilemap.size,
				data: tilemap.data,
				depth_value: depth_index_to_value(tilemap.depth_index, depth_index),
			}
		);

		let fluidmap = fluidmap.map(|fluidmap|
			RenderDrawFluidmap {
				size: fluidmap.size,
				data: fluidmap.data,
				depth_value: depth_index_to_value(fluidmap.depth_index, depth_index),
			}
		);

		RenderDraw {
			clear_color,
			vertex_data,
			vertex_counts,
			texts,
			tilemap,
			fluidmap,
		}
	}
}

pub const fn bytes_per_vertex() -> u64 {
	let position = 3;
	let uv = 2;
	let color = 3;
	let count = position + uv + color;
	let bytes_per_float = 4;

	count * bytes_per_float
}

fn triangles_to_bytes(window_size: SubPixelVec, triangles: &[Triangle], max_depth: DepthIndex) -> Vec<u8> {
	let bytes_per_triangle = 3 * bytes_per_vertex() as usize;
	let bytes_in_triangles = bytes_per_triangle * triangles.len();
	let mut bytes = Vec::<u8>::with_capacity(bytes_in_triangles);

	for triangle in triangles {
		let depth = depth_index_to_value(triangle.depth_index, max_depth);
		for vertex in triangle.vertices.iter().rev() {
			let position = vertex.position.to_surface(window_size);
			let l = [
				position.x.to_le_bytes(),
				position.y.to_le_bytes(),
				depth.to_le_bytes(),
				vertex.uv.x.to_le_bytes(),
				vertex.uv.y.to_le_bytes(),
				vertex.color.r.to_le_bytes(),
				vertex.color.g.to_le_bytes(),
				vertex.color.b.to_le_bytes(),
			];
			bytes.extend(l.iter().flat_map(|x| x.iter()));
		}
	}

	bytes
}

fn depth_index_to_value(index: DepthIndex, max_index: DepthIndex) -> DepthValue {
	(max_index - index + 0.001) / (max_index + 0.002)
}
