import vert_src from '../../../res/web_shader/fluidmap.vert.glsl'
import frag_src from '../../../res/web_shader/fluidmap.frag.glsl'

export let state = {}

export function init() {
	state.program = create_program(gl, vert_src, frag_src)
	state.locations = {}
	state.locations.vertex_position = gl.getAttribLocation(state.program, 'vertex_position')
	state.locations.vertex_uv = gl.getAttribLocation(state.program, 'vertex_uv')
	state.locations.fluidmap_tex = gl.getUniformLocation(state.program, 'fluidmap_tex')
	state.locations.fluidmap_tex_size = gl.getUniformLocation(state.program, 'fluidmap_tex_size')
	state.locations.elapsed_time = gl.getUniformLocation(state.program, 'elapsed_time')
	state.buffer = gl.createBuffer()
	state.fluidmap_texture = gl.createTexture()

	const vertices = [
		-1.0, -1.0, 0.0, 0.0,
		 1.0, -1.0, 1.0, 0.0,
		-1.0,  1.0, 0.0, 1.0,
		 1.0,  1.0, 1.0, 1.0,
	]

	gl.bindBuffer(gl.ARRAY_BUFFER, state.buffer)
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW)

	gl.bindTexture(gl.TEXTURE_2D, state.fluidmap_texture)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST)
}

function create_program(gl, vert_src, frag_src) {
	const vert = create_shader('fluidmap_vert', gl.VERTEX_SHADER, vert_src)
	const frag = create_shader('fluidmap_frag', gl.FRAGMENT_SHADER, frag_src)

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
