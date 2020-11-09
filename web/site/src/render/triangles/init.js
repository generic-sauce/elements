import vert_src from '../../../res/web_shader/triangles.vert.glsl'
import frag_src from '../../../res/web_shader/triangles.frag.glsl'
import { load } from './load_textures'

export let state = {}
state.ready = false

// TODO this is okay but probably should be moved to before_init() because images can be loaded async
export function init(texture_filepaths) {
	load(texture_filepaths, function(textures) {
		do_init(textures)
		state.ready = true
	})
}

function do_init(textures) {
	state.program = create_program(gl, vert_src, frag_src)
	state.locations = {}
	state.locations.vertex_position = gl.getAttribLocation(state.program, 'vertex_position')
	state.locations.vertex_uv = gl.getAttribLocation(state.program, 'vertex_uv')
	state.locations.vertex_color = gl.getAttribLocation(state.program, 'vertex_color')
	state.locations.tex = gl.getUniformLocation(state.program, 'tex')
	state.buffer = gl.createBuffer()
	state.textures = textures // TODO
}

function create_program(gl, vert_src, frag_src) {
	const vert = create_shader('triangles_vert', gl.VERTEX_SHADER, vert_src)
	const frag = create_shader('triangles_frag', gl.FRAGMENT_SHADER, frag_src)

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
