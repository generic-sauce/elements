import vert_src from '../../../res/web_shader/tilemap.vert.glsl'
import frag_src from '../../../res/web_shader/tilemap.frag.glsl'
import { create_program } from '../misc'

export let state = {}

export function init() {
	state.program = create_program("tilemap", vert_src, frag_src)
	state.locations = {}
	state.locations.vertex_position = gl.getAttribLocation(state.program, 'vertex_position')
	state.locations.vertex_uv = gl.getAttribLocation(state.program, 'vertex_uv')
	state.locations.tilemap_tex = gl.getUniformLocation(state.program, 'tilemap_tex')
	state.locations.tilemap_tex_size = gl.getUniformLocation(state.program, 'tilemap_tex_size')
	state.buffer = gl.createBuffer()
	state.tilemap_texture = gl.createTexture()

	gl.bindTexture(gl.TEXTURE_2D, state.tilemap_texture)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST)
}
