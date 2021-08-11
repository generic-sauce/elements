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

	triangles_mod.init(texture_filenames)
	fluidmap_mod.init()
	tilemap_mod.init()
	text_mod.init()
}

function create_view_matrix(camera) {
	// we need the transposed matrix
	return [
		camera.zoom, 0, 0,
		0, camera.zoom, 0,
		camera.left_bot[0] * 2.0, camera.left_bot[1] * 2.0, 1,
	]
}

let normal_camera = {
	left_bot: [0, 0],
	zoom: 1,
}
let normal_view_matrix = create_view_matrix(normal_camera)

export function render(draw) {
	let clear = draw.clear_color
	// gl.clearColor(clear.r, clear.g, clear.b, clear.a)
	// gl.clear(gl.COLOR_BUFFER_BIT)

	triangles_mod.set_vertices(draw.vertex_data)

	let triangle_command_index = 0
	let vertex_count = 0

	let text_command_index = 0

	let view_matrix = normal_view_matrix
	let transformed_view_matrix = create_view_matrix(draw.camera)

	let camera_mode = ""
	function change_camera_mode(mode) {
		if (mode != camera_mode) {
			console.assert(mode == "Normal" || mode == "Transformed", "unknown camera mode '" + camera_mode + "'")
			view_matrix = mode == "Normal" ? normal_view_matrix : transformed_view_matrix
			camera_mode = mode
			return true
		}
	}

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

			if (change_camera_mode(triangle_command.camera_mode))
				triangles_mod.set_view_matrix(view_matrix)

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
