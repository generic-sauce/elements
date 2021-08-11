use crate::prelude::*;

mod vec;
pub use vec::*;

mod camera;
pub use camera::*;

mod player;
use player::*;

mod world;
pub use self::world::*;

mod texture;
pub use texture::*;

mod color;
pub use color::*;

mod render;
pub use render::*;

mod lobby;
pub use lobby::*;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Flip {
	Normal,
	Horizontal,
}

#[derive(Copy, Clone)]
struct Vertex {
	position: ViewVec,
	uv: TextureVec,
	color: Color,
}

pub struct Text {
	pub left_bot: ViewVec,
	pub scale: f32,
	pub color: Color,
	pub string: String,
}

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
#[repr(usize)]
pub enum DrawCommand {
	Tilemap,
	Fluidmap,
	Text,
	Triangles,
}

pub const DRAW_COMMAND_COUNT: usize = 4;

pub type VertexIndex = usize;

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
pub struct TriangleDrawCommand {
	pub texture_index: TextureIndex,
	pub count: VertexIndex,
	pub camera_mode: CameraMode,
}

pub struct DrawTilemap {
	pub size: TileVec,
	pub data: Vec<u8>,
}

impl DrawTilemap {
	pub fn new(tilemap: &TileMap) -> DrawTilemap {
		let size = tilemap.size;
		let data: Vec<u8> = tilemap.iter()
			.map(|p| tilemap.get(p))
			.map(|t| match t {
				Tile::Void => 0,
				Tile::Ground => 1,
				Tile::Wall { owner, .. } => 2 + owner as u8,
			})
			.collect();

		DrawTilemap {
			size,
			data,
		}
	}
}

pub struct DrawFluidmap {
	pub size: FluidVec,
	pub data: Vec<u8>,
}

impl DrawFluidmap {
	pub fn new(fluidmap: &FluidMap) -> DrawFluidmap {
		let size: FluidVec = TileVec::new(128, 72).cast();

		let mut data: Vec<u8> = Vec::new();
		data.resize((4 * size.x * size.y) as usize, 0 as u8);

		for fluid in fluidmap.iter() {
			let cell_id = fluid.position / TILESIZE;
			let local_position = ((fluid.position.x % TILESIZE) as u8, (fluid.position.y % TILESIZE) as u8);

			let cell_index = 4 * (cell_id.x + cell_id.y * size.x as i32) as usize;
			data[cell_index+3] = 255;
			data[cell_index+2] = (fluid.owner * 255) as u8;
			data[cell_index+1] = local_position.1 as u8;
			data[cell_index]   = local_position.0 as u8;
		}

		DrawFluidmap {
			size,
			data,
		}
	}
}

pub struct Draw {
	camera: Option<Camera>,
	clear_color: Option<Color>,
	commands: Vec<DrawCommand>,
	triangle_commands: Vec<TriangleDrawCommand>,
	triangles: Vec<Vertex>,
	tilemap: Option<DrawTilemap>,
	fluidmap: Option<DrawFluidmap>,
	texts: Vec<Text>,
	// unordered_triangles: Vec<Triangles>,
}

impl Draw {
	pub fn new() -> Draw {
		let camera = None;
		let clear_color = None;
		let commands = Vec::new();
		let tilemap = None;
		let fluidmap = None;
		let texts = Vec::new();
		let triangles = Vec::new();
		let triangle_commands = Vec::new();

		Draw {
			camera,
			clear_color,
			commands,
			tilemap,
			fluidmap,
			texts,
			triangles,
			triangle_commands,
		}
	}

	#[allow(unused)]
	pub fn set_clear_color(&mut self, clear_color: Color) {
		if let Some(_) = self.clear_color {
			panic!("clear color was set already");
		}
		self.clear_color = Some(clear_color);
	}

	#[allow(unused)]
	pub fn set_camera(&mut self, camera: Camera) {
		if let Some(_) = self.camera {
			panic!("camera was set already");
		}
		self.camera = Some(camera);
	}

	fn push_triangle_command(&mut self, texture_index: TextureIndex, count: VertexIndex, camera_mode: CameraMode) {
		if let Some(prev) = self.triangle_commands.last_mut() {
			if texture_index == prev.texture_index && camera_mode == prev.camera_mode {
				prev.count += count;
				return;
			}
		}

		self.commands.push(DrawCommand::Triangles);
		self.triangle_commands.push(TriangleDrawCommand {
			texture_index,
			count,
			camera_mode,
		});
	}

	#[allow(unused)]
	pub fn texture<T: IntoDrawVec>(
		&mut self,
		left_bot: T,
		right_top: T,
		texture_index: impl IntoTextureIndex,
		flip: Flip,
		color: Option<Color>,
	) {
		let texture_index = texture_index.into_texture_index();
		let left_bot = left_bot.to_draw();
		let camera_mode = left_bot.camera_mode;
		let left_bot = left_bot.vec;
		let right_top = right_top.to_view();

		let color = color.unwrap_or(Color::WHITE);
		let (left_uv, right_uv) = match flip {
			Flip::Normal => (0.0, 1.0),
			Flip::Horizontal => (1.0, 0.0),
		};

		self.triangles.extend([
			Vertex { position: left_bot,                   uv: TextureVec::new(left_uv, 0.0),  color },
			Vertex { position: v(right_top.x, left_bot.y), uv: TextureVec::new(right_uv, 0.0), color },
			Vertex { position: right_top,                  uv: TextureVec::new(right_uv, 1.0), color },

			Vertex { position: left_bot,                   uv: TextureVec::new(left_uv, 0.0),  color },
			Vertex { position: right_top,                  uv: TextureVec::new(right_uv, 1.0), color },
			Vertex { position: v(left_bot.x, right_top.y), uv: TextureVec::new(left_uv, 1.0),  color },
		].iter().cloned());

		self.push_triangle_command(texture_index, 6, camera_mode);
	}

	#[allow(unused)]
	pub fn rectangle<T: IntoDrawVec>(
		&mut self,
		left_bot: T,
		right_top: T,
		color: Color,
	) {
		let texture_index = TextureId::White.into_texture_index();
		let left_bot = left_bot.to_draw();
		let camera_mode = left_bot.camera_mode;
		let left_bot = left_bot.vec;
		let right_top = right_top.to_view();

		self.triangles.extend([
			Vertex { position: left_bot,                   uv: TextureVec::new(0.0, 0.0), color },
			Vertex { position: v(right_top.x, left_bot.y), uv: TextureVec::new(1.0, 0.0), color },
			Vertex { position: right_top,                  uv: TextureVec::new(1.0, 1.0), color },

			Vertex { position: left_bot,                   uv: TextureVec::new(0.0, 0.0), color },
			Vertex { position: right_top,                  uv: TextureVec::new(1.0, 1.0), color },
			Vertex { position: v(left_bot.x, right_top.y), uv: TextureVec::new(0.0, 1.0), color },
		].iter().cloned());

		self.push_triangle_command(texture_index, 6, camera_mode);
	}

	#[allow(unused)]
	pub fn tilemap(&mut self, tilemap: &TileMap) {
		if let Some(_) = self.tilemap {
			panic!("tilemap was drawn already");
		}

		self.tilemap = Some(DrawTilemap::new(tilemap));
		self.commands.push(DrawCommand::Tilemap);
	}

	#[allow(unused)]
	pub fn fluidmap(&mut self, fluidmap: &FluidMap) {
		if let Some(_) = self.fluidmap {
			panic!("fluidmap was drawn already");
		}

		self.fluidmap = Some(DrawFluidmap::new(fluidmap));
		self.commands.push(DrawCommand::Fluidmap);
	}

	fn circle_frac_iter(points: u32) -> impl Iterator<Item=(f32, f32)> {
		let frac = std::f32::consts::PI * 2.0 / points as f32;
		let i0 = (0..points)
			.map(move |i| frac * i as f32);
		let i1 = (0..points)
			.map(move |i| frac * (i+1) as f32);

		i0.zip(i1)
	}

	#[allow(unused)]
	pub fn circle(
		&mut self,
		center: impl IntoDrawVec,
		scale: f32,
		color: Color,
	) {
		self.arc(center, scale, color, 1.0, 0.0);
	}

	#[allow(unused)]
	pub fn arc(
		&mut self,
		center: impl IntoDrawVec,
		scale: f32,
		color: Color,
		arc_size: f32,
		arc_offset: f32,
	) {
		let points = 32;
		let center = center.to_draw();
		let camera_mode = center.camera_mode;
		let center = center.vec;
		let center_uv = TextureVec::new(0.5, 0.5);
		let arc_offset = arc_offset * std::f32::consts::PI * 2.0;

		let mut vertex_count = 0;
		for (frac0, frac1) in Self::circle_frac_iter(points) {
			let frac0 = frac0 * arc_size + arc_offset;
			let frac1 = frac1 * arc_size + arc_offset;

			let x0 = f32::cos(frac0);
			let y0 = f32::sin(frac0);
			let x1 = f32::cos(frac1);
			let y1 = f32::sin(frac1);

			let point0 = center + CanvasVec::new(x0, y0).to_view() * scale;
			let point1 = center + CanvasVec::new(x1, y1).to_view() * scale;
			let uv0 = TextureVec::new(x0, y0) * 0.5 + 0.5;
			let uv1 = TextureVec::new(x1, y1) * 0.5 + 0.5;

			self.triangles.extend([
				Vertex { position: center, uv: center_uv, color },
				Vertex { position: point0, uv: uv0,       color },
				Vertex { position: point1, uv: uv1,       color },
			].iter().cloned());

			vertex_count += 3;
		}

		let texture_index = TextureId::White.into_texture_index();
		self.push_triangle_command(texture_index, vertex_count, camera_mode);
	}

	#[allow(unused)]
	pub fn text(
		&mut self,
		left_bot: impl IntoViewVec,
		scale: f32,
		color: Color,
		string: &str,
	) {
		let left_bot = left_bot.to_view();
		let string = string.to_string();

		let text = Text {
			left_bot,
			scale,
			color,
			string,
		};

		self.texts.push(text);
		self.commands.push(DrawCommand::Text);
	}
}
