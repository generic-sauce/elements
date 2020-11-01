use crate::prelude::*;

const IMG_SIZE: i32 = 64;
const IMG_PLAYER_HEIGHT: i32 = 54;
const CURSOR_INDICATOR_RADIUS: i32 = TILESIZE / 2;

// the texture rect has been obtained by reading the .png file

fn texture_center(pl: &Player) -> GameVec {
	GameVec::new(
		pl.left_bot.x + PLAYER_SIZE.x / 2,
		pl.left_bot.y + PLAYER_SIZE.y * IMG_SIZE / 2 / IMG_PLAYER_HEIGHT,
	)
}

fn texture_radius() -> GameVec {
	let r = PLAYER_SIZE.y * IMG_SIZE / IMG_PLAYER_HEIGHT / 2;
	GameVec::new(r, r)
}

const RADIUS: i32 = PLAYER_SIZE.y * IMG_SIZE / IMG_PLAYER_HEIGHT / 2;

pub(in super) fn draw_players(draw: &mut Draw, world: &World) {
	for p in &world.players {
		let center = p.center_position();
		let left_bot = GameVec::new(center.x - RADIUS, p.left_bot.y);
		let right_top = GameVec::new(center.x + RADIUS, p.left_bot.y + RADIUS * 2);
		let flip = if p.direction == PlayerDirection::Right { Flip2::Normal } else { Flip2::Horizontal };
		draw.texture(left_bot, right_top, p.animation, flip, None);
	}
}

pub(in super) fn draw_cursors(draw: &mut Draw, world: &World) {
	for p in &world.players {
		let radius = TILESIZE / 2;
		let radius = GameVec::new(radius, radius);
		let center = p.cursor_position();
		let left_bot = center - radius;
		let right_top = center + radius;
		draw.rectangle(left_bot, right_top, wgpu::Color::BLACK);
	}
}

pub(in super) fn draw_healthbars(draw: &mut Draw, world: &World) {
	for p in &world.players {
		let mut size = GameVec::new(PLAYER_SIZE.x, TILESIZE / 2);
		let left_bot = p.left_bot + GameVec::new(0, PLAYER_SIZE.y + TILESIZE);
		draw.rectangle(left_bot, left_bot + size, wgpu::Color::BLACK);
		size.x = (size.x as f32 * (p.health as f32 / MAX_HEALTH as f32)) as i32;
		draw.rectangle(left_bot, left_bot + size, wgpu::Color::GREEN);
	}
}
