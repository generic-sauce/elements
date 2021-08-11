import vert_src from '../../../res/web_shader/triangles.vert.glsl'
import frag_src from '../../../res/web_shader/triangles.frag.glsl'
import { create_program, res_image, load_texture } from '../misc'

export let state = {}

// TODO this is okay but probably should be moved to before_init() because images can be loaded async
export function init(texture_filenames) {
	let textures = []
	for (let filename of texture_filenames)
		textures.push(load_texture(filename))

	state.program = create_program("triangles", vert_src, frag_src)
	state.locations = {}
	state.locations.vertex_position = gl.getAttribLocation(state.program, 'vertex_position')
	state.locations.vertex_uv = gl.getAttribLocation(state.program, 'vertex_uv')
	state.locations.vertex_color = gl.getAttribLocation(state.program, 'vertex_color')
	state.locations.tex = gl.getUniformLocation(state.program, 'tex')
	state.locations.v = gl.getUniformLocation(state.program, 'v')
	state.buffer = gl.createBuffer()
	state.textures = textures
}
