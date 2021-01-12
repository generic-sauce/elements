import { state } from "./init.js"

export function render(draw) {
	gl.useProgram(state.program)

	update_fluidmap_tex(draw)
	gl.uniform1f(state.locations.elapsed_time, performance.now() / 1000)

	const vertices = [
		-1.0, -1.0, 0.0, 0.0,
		 1.0, -1.0, 1.0, 0.0,
		-1.0,  1.0, 0.0, 1.0,
		 1.0,  1.0, 1.0, 1.0,
	]

	gl.bindBuffer(gl.ARRAY_BUFFER, state.buffer)
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW)

	{ // vertex_position
		const count = 2;
		const type = gl.FLOAT;
		const normalize = false;
		const stride = 4 * 4;
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
		const stride = 4 * 4;
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

	{
		const offset = 0
		const count = 4
		gl.drawArrays(gl.TRIANGLE_STRIP, offset, count)
	}
}

function update_fluidmap_tex(draw) {
	const width = draw.fluidmap.size[0]
	const height = draw.fluidmap.size[1]
	const data = draw.fluidmap_data

	gl.bindTexture(gl.TEXTURE_2D, state.fluidmap_texture)
	gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, width, height, 0, gl.RGBA, gl.UNSIGNED_BYTE, data)
	gl.uniform2f(state.locations.fluidmap_tex_size, width, height)
}
