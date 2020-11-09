import * as fluidmap_mod from "./fluidmap/mod.js"
import * as tilemap_mod from "./tilemap/mod.js"
// import * as playermod from "./player/mod.js"

export function init() {
	window.canvas = document.getElementById("canvas");
	window.gl = canvas.getContext("webgl");

	onresize = update_canvas_size
	update_canvas_size()

	gl.clearColor(0.8, 0.8, 1.0, 1.0)

	fluidmap_mod.init()
	tilemap_mod.init()
	// playermod.init()
}

export function render(draw) {
	gl.clear(gl.COLOR_BUFFER_BIT)

	fluidmap_mod.render(draw)
	tilemap_mod.render(draw)
	// playermod.draw()
}

function update_canvas_size() {
	canvas.width = window.innerWidth
	canvas.height = window.innerHeight
	gl.viewport(0, 0, canvas.width, canvas.height)
}
