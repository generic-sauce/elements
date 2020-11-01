use crate::prelude::*;

pub struct GraphicsWorld {
	pub tilemap_size: TileVec,
	pub tilemap_data: Vec<u8>,
	pub fluidmap_data: Vec<u8>,
	pub players: [Player; 2],
	pub elapsed_time: Duration,
}

impl GraphicsWorld {
	pub fn new(tilemap: &TileMap, fluidmap: &FluidMap, players: [Player; 2], elapsed_time: Duration) -> GraphicsWorld {

		let tilemap_data: Vec<u8> = tilemap.iter()
			.map(|p| tilemap.get(p))
			.map(|t| match t {
				Tile::Void => 0,
				Tile::Ground => 1,
				Tile::Wall { owner, .. } => 2 + owner as u8,
			})
			.collect();

		let mut fluidmap_data: Vec<u8> = Vec::new();
		fluidmap_data.resize((4 * tilemap.size.x * tilemap.size.y) as usize, 0 as u8);

		for fluid in fluidmap.iter() {
			let cell_id = fluid.position / TILESIZE;
			let local_position = ((fluid.position.x % TILESIZE) as u8, (fluid.position.y % TILESIZE) as u8);

			let cell_index = 4 * (cell_id.x + cell_id.y * tilemap.size.x as i32) as usize;
			fluidmap_data[cell_index+3] = 255;
			fluidmap_data[cell_index+2] = (fluid.owner * 255) as u8;
			fluidmap_data[cell_index+1] = local_position.1 as u8;
			fluidmap_data[cell_index+0] = local_position.0 as u8;
		}

		GraphicsWorld {
			tilemap_size: tilemap.size,
			tilemap_data,
			fluidmap_data,
			players,
			elapsed_time,
		}
	}
}

const IMG_SIZE: i32 = 64;
const IMG_PLAYER_HEIGHT: i32 = 54;
const RADIUS: i32 = PLAYER_SIZE.y * IMG_SIZE / IMG_PLAYER_HEIGHT / 2;

impl Graphics {
	pub(in crate::graphics) fn draw_players(&mut self, draw: &mut Draw, world: &GraphicsWorld) {
		for p in &world.players {
			let center = p.center_position();
			let left_bot = GameVec::new(center.x - RADIUS, p.left_bot.y);
			let right_top = GameVec::new(center.x + RADIUS, p.left_bot.y + RADIUS * 2);
			let flip = if p.direction == PlayerDirection::Right { Flip2::Normal } else { Flip2::Horizontal };
			// self.triangles.draw_sprite(context, left_bot, right_top, TextureId2::BluePlayerIdle1, None);
			draw.texture(left_bot, right_top, p.animation, flip, None);
		}
	}

	pub(in crate::graphics) fn draw_cursors(&mut self, draw: &mut Draw, world: &GraphicsWorld) {
		for p in &world.players {
			let radius = TILESIZE / 2;
			let radius = GameVec::new(radius, radius);
			let center = p.cursor_position();
			let left_bot = center - radius;
			let right_top = center + radius;
			draw.rectangle(left_bot, right_top, wgpu::Color::BLACK);
		}
	}

	pub(in crate::graphics) fn draw_healthbars(&mut self, draw: &mut Draw, world: &GraphicsWorld) {
		for p in &world.players {
			let mut size = GameVec::new(PLAYER_SIZE.x, TILESIZE / 2);
			let left_bot = p.left_bot + GameVec::new(0, PLAYER_SIZE.y + TILESIZE);
			draw.rectangle(left_bot, left_bot + size, wgpu::Color::BLACK);
			size.x = (size.x as f32 * (p.health as f32 / MAX_HEALTH as f32)) as i32;
			draw.rectangle(left_bot, left_bot + size, wgpu::Color::GREEN);
		}
	}
}
