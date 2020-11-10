import * as render_mod from "./render/mod.js"
import * as inputmod from "./input.js"
import * as tilemapmod from "./tilemap.js"

window.e2 = {}

window.before_init = function() {
}

window.js_init = render_mod.init

window.js_render = function(draw, tilemap_data, fluidmap_data, vertex_data) {
	draw.tilemap_data = tilemap_data
	draw.fluidmap_data = fluidmap_data
	draw.vertex_data = vertex_data

	render_mod.render(draw)
}

window.input_state = function(i) {
	return inputmod.calc_input_state(i)
}

window.load_tilemap = function(src, cb) {
	tilemapmod.load(src, cb)
}

import("../node_modules/elements/elements.js")
