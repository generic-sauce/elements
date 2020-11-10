import * as fluidmap_mod from "./fluidmap/mod.js"
import * as tilemap_mod from "./tilemap/mod.js"
import * as triangles_mod from "./triangles/mod.js"

export function init(texture_filenames) {
	window.canvas = document.getElementById("canvas");
	window.gl = canvas.getContext("webgl");

	gl.enable(gl.DEPTH_TEST)
	gl.depthFunc(gl.LESS)

	onresize = update_canvas_size
	update_canvas_size()

	fluidmap_mod.init()
	tilemap_mod.init()
	triangles_mod.init(texture_filenames)
}

export function render(draw) {
	let clear = draw.clear_color
	gl.clearColor(clear.r, clear.g, clear.b, clear.a)
	gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)

	triangles_mod.render(draw)
	tilemap_mod.render(draw)
	fluidmap_mod.render(draw)
}

function update_canvas_size() {
	canvas.width = window.innerWidth
	canvas.height = window.innerHeight
	gl.viewport(0, 0, canvas.width, canvas.height)
}
