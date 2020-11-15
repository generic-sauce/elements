import * as inputmod from "./input.js"
import * as tilemapmod from "./tilemap.js"

console.log("a?");

this.js_init = function(texture_filenames) {
	console.log("woo?");
	console.log("herewego:",texture_filenames);
	this.postMessage({
		type: "init",
		texture_filenames,
	});
}

this.js_render = function(draw, tilemap_data, fluidmap_data, vertex_data) {
	draw.tilemap_data = tilemap_data
	draw.fluidmap_data = fluidmap_data
	draw.vertex_data = vertex_data

	postMessage({
		type: "render",
		draw,
	});
}

this.input_state = function(i) {
	return inputmod.calc_input_state(i)
}

this.load_tilemap = function(src, cb) {
	tilemapmod.load(src, cb)
}

import("../../node_modules/elements/elements.js") // TODO use web/pkg-path without linking
