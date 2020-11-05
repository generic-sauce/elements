import * as drawmod from "./draw/mod.js";
import * as inputmod from "./input.js";
import * as tilemapmod from "./tilemap.js";

window.e2 = {};
window.init_js = function(f) {
	drawmod.init();
};
window.draw_render_world = function(rw, tilemap_data, fluidmap_data) {
	e2.render_world = rw;
	e2.render_world.tilemap_data = tilemap_data;
	e2.render_world.fluidmap_data = fluidmap_data;

	drawmod.draw();
};
window.input_state = function(i) {
	return inputmod.calc_input_state(i);
};
window.load_tilemap = function(src) {
	return tilemapmod.load(src);
};

import("../node_modules/elements2/elements2.js");
