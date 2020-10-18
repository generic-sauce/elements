export function player_rect(i) {
	const lb = e2.render_world.players[i].left_bot;
	const s = e2.render_world.player_size;
	const game_rect = [
		[lb[0]       , lb[1]],
		[lb[0] + s[0], lb[1]],
		[lb[0]       , lb[1] + s[1]],
		[lb[0] + s[0], lb[1] + s[1]],
	];
	let screen_rect = game_rect.map(game_to_screen_point);
	return [
		screen_rect[0][0], screen_rect[0][1],
		screen_rect[1][0], screen_rect[1][1],
		screen_rect[2][0], screen_rect[2][1],
		screen_rect[3][0], screen_rect[3][1],
	];

}

function max_game_point() {
	return e2.render_world.tilemap_size
		.map(x => x * 256);
}

function game_to_screen_point(p) {
	const max = max_game_point();
	return [p[0] / max[0], p[1] / max[1]]
		.map(x => 2*x - 1);
}
