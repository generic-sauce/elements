import { state } from "./init.js"

export function render(draw) {
	if (!state.ready)
		return

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

	{ // vertex_uv
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
	for (let i = 0; i < counts; ++i) {
		gl.bindTexture(gl.TEXTURE_2D, state.triangles_texture)
		const count = counts[i]
		gl.drawArrays(gl.TRIANGLE_STRIP, offset, count)
		offset += count
	}
}

function update_vertices(draw) {
	// let vertices = // TODO

	console.log(draw.triangles)
	alert()

	gl.bindBuffer(gl.ARRAY_BUFFER, state.buffer)
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW)
}
