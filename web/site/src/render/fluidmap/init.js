import vert_src from '../../../res/web_shader/fluidmap.vert.glsl'
import frag_src from '../../../res/web_shader/fluidmap.frag.glsl'
import { create_program } from '../misc'

export let state = {}

export function init() {
	state.program = create_program("fluidmap", vert_src, frag_src)
	state.locations = {}
	state.locations.vertex_position = gl.getAttribLocation(state.program, 'vertex_position')
	state.locations.vertex_uv = gl.getAttribLocation(state.program, 'vertex_uv')
	state.locations.fluidmap_tex = gl.getUniformLocation(state.program, 'fluidmap_tex')
	state.locations.fluidmap_tex_size = gl.getUniformLocation(state.program, 'fluidmap_tex_size')
	state.locations.elapsed_time = gl.getUniformLocation(state.program, 'elapsed_time')
	state.buffer = gl.createBuffer()
	state.fluidmap_texture = gl.createTexture()

	gl.bindTexture(gl.TEXTURE_2D, state.fluidmap_texture)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST)
}
