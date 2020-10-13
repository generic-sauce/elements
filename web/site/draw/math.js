e2.max_game_point = function() {
	return e2.world.tilemap.size
		.map(x => x * 256);
}

e2.game_to_screen_point = function(p) {
	const max = e2.max_game_point();
	return [p[0] / max[0], p[1] / max[1]]
		.map(x => 2*x - 1);
}

e2.player_rect = function(i) {
	const lb = e2.world.players[i].left_bot;
	const s = e2.cnst.PLAYER_SIZE;
	const game_rect = [
		[lb[0]       , lb[1]],
		[lb[0] + s[0], lb[1]],
		[lb[0]       , lb[1] + s[1]],
		[lb[0] + s[0], lb[1] + s[1]],
	];
	let screen_rect = game_rect.map(e2.game_to_screen_point);
	return [
		screen_rect[0][0], screen_rect[0][1],
		screen_rect[1][0], screen_rect[1][1],
		screen_rect[2][0], screen_rect[2][1],
		screen_rect[3][0], screen_rect[3][1],
	];

}
