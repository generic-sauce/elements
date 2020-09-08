use crate::prelude::*;

const IMG_SIZE: i32 = 64;
const IMG_PLAYER_WIDTH: i32 = 22;
const IMG_PLAYER_HEIGHT: i32 = 54;
const IMG_PLAYER_SIZE: GameVec = GameVec::new(IMG_PLAYER_WIDTH, IMG_PLAYER_HEIGHT);

const CURSOR_INDICATOR_RADIUS: i32 = TILESIZE / 2;

// the texture rect has been obtained by reading the .png file

fn texture_center(pl: &Player) -> GameVec {
	GameVec::new(
		pl.left_bot.x + PLAYER_SIZE.x / 2,
		pl.left_bot.y + PLAYER_SIZE.y * IMG_SIZE / 2 / IMG_PLAYER_HEIGHT,
	)
}

fn texture_radius() -> GameVec {
	PLAYER_SIZE * GameVec::new(IMG_SIZE, IMG_SIZE) / IMG_PLAYER_SIZE / 2
}

pub(in super) fn draw(pl: &Player, target: &impl RenderTarget, context: &DrawContext) {
	// character
	let flip = if let PlayerDirection::Right = pl.direction { Flip::Normal } else { Flip::Horizontal };
	context.draw_animation(target, texture_center(pl), texture_radius(), pl.animation, flip);

	// cursor
	context.draw_circle(target, pl.center_position() + pl.cursor, CURSOR_INDICATOR_RADIUS, Color::BLACK);
}
