import { state } from "./init.js"

export function set_vertices(vertex_data) {
	gl.useProgram(state.program)
	gl.bindBuffer(gl.ARRAY_BUFFER, state.buffer)
	gl.bufferData(gl.ARRAY_BUFFER, vertex_data, gl.STATIC_DRAW)
}

export function render(texture_index, from, to) {
	gl.useProgram(state.program)
	gl.bindBuffer(gl.ARRAY_BUFFER, state.buffer)

	{ // vertex_position
		const count = 2
		const type = gl.FLOAT
		const normalize = false
		const stride = 8 * 4
		const offset = 0 * 4
		gl.vertexAttribPointer(
			state.locations.vertex_position,
			count,
			type,
			normalize,
			stride,
			offset
		)
		gl.enableVertexAttribArray(state.locations.vertex_position)
	}

	{ // vertex_uv
		const count = 2
		const type = gl.FLOAT
		const normalize = false
		const stride = 8 * 4
		const offset = 2 * 4
		gl.vertexAttribPointer(
			state.locations.vertex_uv,
			count,
			type,
			normalize,
			stride,
			offset
		)
		gl.enableVertexAttribArray(state.locations.vertex_uv)
	}

	{ // vertex_color
		const count = 4
		const type = gl.FLOAT
		const normalize = false
		const stride = 8 * 4
		const offset = 4 * 4
		gl.vertexAttribPointer(
			state.locations.vertex_color,
			count,
			type,
			normalize,
			stride,
			offset
		)
		gl.enableVertexAttribArray(state.locations.vertex_color)
	}

	gl.bindTexture(gl.TEXTURE_2D, state.textures[texture_index])
	gl.drawArrays(gl.TRIANGLES, from, to - from)
}
