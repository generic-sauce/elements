export function create_program(name, vert_src, frag_src) {
	const vert = create_shader(name + "_vert", gl.VERTEX_SHADER, vert_src)
	const frag = create_shader(name + "_frag", gl.FRAGMENT_SHADER, frag_src)

	const program = gl.createProgram()
	gl.attachShader(program, vert)
	gl.attachShader(program, frag)
	gl.linkProgram(program)
	gl.deleteShader(vert)
	gl.deleteShader(frag)

	if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
		alert("Unable to initialize the shader program: " + gl.getProgramInfoLog(program))
		return null
	}

	return program
}

export function create_shader(name, type, src) {
	const shader = gl.createShader(type)
	gl.shaderSource(shader, src)
	gl.compileShader(shader)

	if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
		alert("An error occurred compiling the shader (" + name + "): " + gl.getShaderInfoLog(shader))
		gl.deleteShader(shader)
		return null
	}

	return shader
}

const unloaded_texture_width = 2
const unloaded_texture_height = 2
const unloaded_texture_data = new Uint8Array([
	100, 0, 100, 255,
	200, 0, 200, 255,
	200, 0, 200, 255,
	100, 0, 100, 255,
])

export function load_texture(url) {
	let texture = gl.createTexture()
	gl.bindTexture(gl.TEXTURE_2D, texture)

	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST)

	const level = 0
	const border = 0
	const src_type = gl.UNSIGNED_BYTE
	const src_format = gl.RGBA
	const dst_format = gl.RGBA

	const width = unloaded_texture_width
	const height = unloaded_texture_height
	const data = unloaded_texture_data
	gl.texImage2D(gl.TEXTURE_2D, level, dst_format, width, height, border, src_format, src_type, data)

	const image = new Image()
	image.onload = function() {
		gl.bindTexture(gl.TEXTURE_2D, texture)
    gl.texImage2D(gl.TEXTURE_2D, level, dst_format, src_format, src_type, image)
	}
	image.src = url

	return texture
}

export function res_image(filename) {
	return "res/images/" + filename
}
