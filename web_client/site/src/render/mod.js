import * as fluidmap_mod from "./fluidmap/mod.js"
import * as tilemap_mod from "./tilemap/mod.js"
import * as triangles_mod from "./triangles/mod.js"
import * as text_mod from "./text/mod.js"

export function init(texture_filenames) {
	window.canvas = document.getElementById("canvas");
	window.gl = canvas.getContext("webgl");

	gl.enable(gl.BLEND)
	gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA)
	gl.blendEquation(gl.FUNC_ADD)

	onresize = update_canvas_size
	update_canvas_quality(0.618)

	fluidmap_mod.init()
	tilemap_mod.init()
	triangles_mod.init(texture_filenames)
	text_mod.init()
}

export function render(draw) {
	let clear = draw.clear_color
	gl.clearColor(clear.r, clear.g, clear.b, clear.a)
	// gl.clear(gl.COLOR_BUFFER_BIT)

	triangles_mod.set_vertices(draw.vertex_data)

	let triangle_command_index = 0
	let vertex_count = 0

	let text_command_index = 0

	for (let command of draw.commands) {
		switch (command) {
		case "Triangles":
			let triangle_command = draw.triangle_commands[triangle_command_index]
			let texture_index = triangle_command.texture_index
			let count = triangle_command.count
			let from = vertex_count
			let to = vertex_count + count
			vertex_count += count
			++triangle_command_index

			triangles_mod.render(texture_index, from, to)
			break;
		case "Text":
			let text = draw.texts[text_command_index]
			text_mod.render(text)
			++text_command_index
			break;
		case "Tilemap":
			tilemap_mod.render(draw)
			break;
		case "Fluidmap":
			fluidmap_mod.render(draw)
			break;
		}
	}
}

function update_canvas_quality(quality) {
	window.canvas_quality = quality
	let inverse = 1 / quality
	window.canvas.style.transform = "scale("+inverse+", "+inverse+") translate(0, "+(quality/-2+0.5)*100+"%)";
	update_canvas_size()
}

function update_canvas_size() {
	canvas.width = window.innerWidth * window.canvas_quality
	canvas.height = window.innerHeight * window.canvas_quality

	let ratio = (canvas.width / canvas.height) / (16 / 9)
	if (ratio > 1.0) {
		canvas.width /= ratio
	} else {
		canvas.height *= ratio
	}

	gl.viewport(0, 0, canvas.width, canvas.height)
}
