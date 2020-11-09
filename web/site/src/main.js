import * as render_mod from "./render/mod.js"
import * as inputmod from "./input.js"
import * as tilemapmod from "./tilemap.js"

window.e2 = {}

window.before_init = function() {
}

window.js_init = render_mod.init

window.js_render = render_mod.render

window.input_state = function(i) {
	return inputmod.calc_input_state(i)
}

window.load_tilemap = function(src, cb) {
	tilemapmod.load(src, cb)
}

import("../node_modules/elements/elements.js")
