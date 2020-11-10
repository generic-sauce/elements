import { state } from "./init.js"

export function render(draw) {
	gl.useProgram(state.program)

	gl.bindBuffer(gl.ARRAY_BUFFER, state.buffer)
	gl.bufferData(gl.ARRAY_BUFFER, draw.vertex_data, gl.STATIC_DRAW)

	{ // vertex_position
		const count = 3;
		const type = gl.FLOAT;
		const normalize = false;
		const stride = 8 * 4;
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
		const stride = 8 * 4;
		const offset = 3 * 4;
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
		const stride = 8 * 4;
		const offset = 5 * 4;
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
	for (let i = 0; i < draw.vertex_counts.length; ++i) {
		const count = draw.vertex_counts[i]
		gl.bindTexture(gl.TEXTURE_2D, state.textures[i])
		gl.drawArrays(gl.TRIANGLES, offset, count)
		offset += count
	}
}

function update_vertices(draw) {
}
