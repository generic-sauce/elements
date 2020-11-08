import { vert_src, frag_src } from "./shader.js"

export let state = {}

export function init() {
	// const vert_src = document.getElementById('tilemap_vert').text
	// const frag_src = document.getElementById('tilemap_frag').text

	state.program = create_program(gl, vert_src, frag_src)
	gl.useProgram(state.program)
	state.locations = {}
	state.locations.vertex_position = gl.getAttribLocation(state.program, 'vertex_position')
	state.locations.vertex_uv = gl.getAttribLocation(state.program, 'vertex_uv')
	state.locations.tilemap_tex = gl.getUniformLocation(state.program, 'tilemap_tex')
	state.locations.tilemap_tex_size = gl.getUniformLocation(state.program, 'tilemap_tex_size')
	state.buffer = gl.createBuffer()
	state.tilemap_texture = gl.createTexture()

	const vertices = [
		-1.0, -1.0, 0.0, 0.0,
		 1.0, -1.0, 1.0, 0.0,
		-1.0,  1.0, 0.0, 1.0,
		 1.0,  1.0, 1.0, 1.0,
	]

	gl.bindBuffer(gl.ARRAY_BUFFER, state.buffer)
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW)

	gl.bindTexture(gl.TEXTURE_2D, state.tilemap_texture)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST)
}

function create_program(gl, vert_src, frag_src) {
	const vert = create_shader("tilemap_vert", gl.VERTEX_SHADER, vert_src)
	const frag = create_shader("tilemap_frag", gl.FRAGMENT_SHADER, frag_src)

	const program = gl.createProgram()
	gl.attachShader(program, vert)
	gl.attachShader(program, frag)
	gl.linkProgram(program)
	// gl.deleteShader(vert)
	// gl.deleteShader(frag)

	if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
		alert('Unable to initialize the shader program: ' + gl.getProgramInfoLog(program))
		return null
	}

	return program
}

function create_shader(name, type, src) {
	const shader = gl.createShader(type)
	gl.shaderSource(shader, src)
	gl.compileShader(shader)

	if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
		alert('An error occurred compiling the shader (' + name + '): ' + gl.getShaderInfoLog(shader))
		gl.deleteShader(shader)
		return null
	}

	return shader
}
