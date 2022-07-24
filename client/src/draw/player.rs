use crate::prelude::*;

const IMG_SIZE: i32 = 64;
const IMG_PLAYER_HEIGHT: i32 = 54;
const RADIUS: i32 = PLAYER_SIZE.y * IMG_SIZE / IMG_PLAYER_HEIGHT / 2;

// the texture rect has been obtained by reading the .png file

// fn texture_center(pl: &Player) -> GameVec {
// 	GameVec::new(
// 		pl.left_bot.x + PLAYER_SIZE.x / 2,
// 		pl.left_bot.y + PLAYER_SIZE.y * IMG_SIZE / 2 / IMG_PLAYER_HEIGHT,
// 	)
// }
//
// fn texture_radius() -> GameVec {
// 	let r = PLAYER_SIZE.y * IMG_SIZE / IMG_PLAYER_HEIGHT / 2;
// 	GameVec::new(r, r)
// }

pub(in super) fn draw_players(draw: &mut Draw, world: &World) {
	for p in &world.players {
		let center = p.center_position();
		let left_bot = GameVec::new(center.x - RADIUS, p.left_bot.y);
		let right_top = GameVec::new(center.x + RADIUS, p.left_bot.y + RADIUS * 2);
		let flip = if p.direction == PlayerDirection::Right { Flip::Normal } else { Flip::Horizontal };
		draw.texture(left_bot, right_top, p.animation, flip, None);
	}
}

pub(in super) fn draw_cursors(draw: &mut Draw, world: &World) {
	for p in &world.players {
		let position = p.cursor_position();
		draw.circle(position, CURSOR_RADIUS, Color::BLACK);
	}
}

pub(in super) fn draw_healthbars(draw: &mut Draw, world: &World) {
	for p in &world.players {
		let size = GameVec::new(PLAYER_SIZE.x * 2, TILESIZE / 2);
		let left_bot = p.left_bot + GameVec::new(-PLAYER_SIZE.x / 2, PLAYER_SIZE.y + TILESIZE);

		// draw surrounding
		let offset = GameVec::new(32, 32);
		draw.rectangle(left_bot - offset, left_bot + size + offset, Color::GRAY);

		// draw black background
		if p.health <= 0 {
			let ratio = (p.health_bar_status.red_death_counter as f32 / RED_HEALTH_DEATH_DURATION as f32).max(0.0);
			draw.rectangle(left_bot, left_bot + size, Color::RED * ratio);
		} else {
			draw.rectangle(left_bot, left_bot + size, Color::BLACK);
		}

		// draw delayed health
		let delayed_health_size = GameVec::new((size.x as f32 * (p.health_bar_status.health_delayed as f32 / MAX_HEALTH as f32)) as i32, size.y);
		draw.rectangle(left_bot, left_bot + delayed_health_size, Color::WHITE);

		// draw bar
		let health_size = GameVec::new((size.x as f32 * (p.health as f32 / MAX_HEALTH as f32)) as i32, size.y);
		draw.rectangle(left_bot, left_bot + health_size, Color::GREEN);
	}
}
