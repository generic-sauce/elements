import vert_src from '../../../res/web_shader/triangles.vert.glsl'
import frag_src from '../../../res/web_shader/triangles.frag.glsl'
import { create_program } from '../misc'

export let state = {}

export function init() {
	state.buffer = gl.createBuffer()

	state.texture = gl.createTexture()
	gl.bindTexture(gl.TEXTURE_2D, state.texture)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST)
}
