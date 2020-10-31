mod texture_state2;
pub use texture_state2::*;

pub(super) mod draw_triangles;
pub(super) mod draw_tilemap;
pub(super) mod draw_fluidmap;

use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct Vertex {
	pub position: SurfaceVec,
	pub uv: Vec2f,
	pub color: wgpu::Color,
}

pub type Triangle = [Vertex; 3];
pub type Triangles = Vec<Triangle>;
pub type TextureTriangles = Vec<Triangles>;

pub struct Draw {
	window_size: Vec2u,
	pub triangles: TextureTriangles,
}

impl Draw {
	pub fn new(window_size: Vec2u, texture_count: usize) -> Draw {
		let mut triangles = TextureTriangles::new();
		triangles.resize_with(texture_count, Default::default);
		Draw {
			window_size,
			triangles,
		}
	}

	#[allow(unused)]
	pub fn texture(
		&mut self,
		left_bot: impl IntoSurfaceVec,
		right_top: impl IntoSurfaceVec,
		texture_index: impl IntoTextureIndex,
		flip: Flip2,
		color: Option<wgpu::Color>
	) {
		let texture_index = texture_index.into_texture_index();
		let triangles = &mut self.triangles[texture_index];
		let left_bot = left_bot.to_surface(self.window_size);
		let right_top = right_top.to_surface(self.window_size);
		let color = if let Some(color) = color { color } else { wgpu::Color::WHITE };
		let (left_uv, right_uv) = match flip {
			Flip2::Normal => (0.0, 1.0),
			Flip2::Horizontal => (1.0, 0.0),
		};

		triangles.push([
			Vertex { position: left_bot, uv: Vec2f::new(left_uv, 0.0), color: color },
			Vertex { position: v(right_top.x, left_bot.y), uv: Vec2f::new(right_uv, 0.0), color: color },
			Vertex { position: right_top, uv: Vec2f::new(right_uv, 1.0), color: color },
		]);

		triangles.push([
			Vertex { position: left_bot, uv: Vec2f::new(left_uv, 0.0), color: color },
			Vertex { position: right_top, uv: Vec2f::new(right_uv, 1.0), color: color },
			Vertex { position: v(left_bot.x, right_top.y), uv: Vec2f::new(left_uv, 1.0), color: color },
		]);
	}

	#[allow(unused)]
	pub fn rectangle(
		&mut self,
		left_bot: impl IntoSurfaceVec,
		right_top: impl IntoSurfaceVec,
		color: wgpu::Color
	) {
		let triangles = &mut self.triangles[TextureId2::White as usize];
		let left_bot = left_bot.to_surface(self.window_size);
		let right_top = right_top.to_surface(self.window_size);

		triangles.push([
			Vertex { position: left_bot, uv: Vec2f::new(0.0, 0.0), color: color },
			Vertex { position: v(right_top.x, left_bot.y), uv: Vec2f::new(1.0, 0.0), color: color },
			Vertex { position: right_top, uv: Vec2f::new(1.0, 1.0), color: color },
		]);

		triangles.push([
			Vertex { position: left_bot, uv: Vec2f::new(0.0, 0.0), color: color },
			Vertex { position: right_top, uv: Vec2f::new(1.0, 1.0), color: color },
			Vertex { position: v(left_bot.x, right_top.y), uv: Vec2f::new(0.0, 1.0), color: color },
		]);
	}
}
