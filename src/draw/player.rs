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

pub(in super) fn draw(pl: &Player, target: &impl RenderTarget, context: &GameDrawContext) {
	// character
	let flip = if let PlayerDirection::Right = pl.direction { Flip::Normal } else { Flip::Horizontal };
	context.draw_animation(target, texture_center(pl), texture_radius(), pl.animation, flip);

	// cursor
	context.draw_circle(target, pl.center_position() + pl.cursor, CURSOR_INDICATOR_RADIUS, Color::BLACK);
}
