import { state } from "./init.js"

export function render(draw) {
	gl.useProgram(state.program)

	let counts = update_vertices(draw)

	{ // vertex_position
		const count = 2;
		const type = gl.FLOAT;
		const normalize = false;
		const stride = 7 * 4;
		const offset = 0 * 4;
		gl.vertexAttribPointer(
			state.locations.vertex_position,
			count,
			type,
			normalize,
			stride,
			offset
		);
		gl.enableVertexAttribArray(state.locations.vertex_position);
	}

	{ // vertex_uv
		const count = 2;
		const type = gl.FLOAT;
		const normalize = false;
		const stride = 7 * 4;
		const offset = 2 * 4;
		gl.vertexAttribPointer(
			state.locations.vertex_uv,
			count,
			type,
			normalize,
			stride,
			offset
		);
		gl.enableVertexAttribArray(state.locations.vertex_uv);
	}

	{ // vertex_color
		const count = 3;
		const type = gl.FLOAT;
		const normalize = false;
		const stride = 7 * 4;
		const offset = 4 * 4;
		gl.vertexAttribPointer(
			state.locations.vertex_color,
			count,
			type,
			normalize,
			stride,
			offset
		);
		gl.enableVertexAttribArray(state.locations.vertex_color);
	}

	let offset = 0
	for (let i = 0; i < counts.length; ++i) {
		const count = counts[i]
		gl.bindTexture(gl.TEXTURE_2D, state.textures[i])
		gl.drawArrays(gl.TRIANGLES, offset, count)
		offset += count
	}
}

function update_vertices(draw) {
	const counts = [3, 3]

	const vertices = [
		0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
		0.5, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0,
		0.0, 0.5, 0.0, 1.0, 1.0, 1.0, 1.0,

		1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
		1.0, 0.5, 1.0, 0.0, 1.0, 1.0, 1.0,
		0.5, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
	]

	gl.bindBuffer(gl.ARRAY_BUFFER, state.buffer)
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW)

	return counts
}
