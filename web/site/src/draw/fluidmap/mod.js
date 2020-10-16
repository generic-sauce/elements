import { init, state } from "./init.js";

export { init, draw }

function draw() {
	const { programInfo, vsBuffer, mapTexture } = state;
	const gl = e2.gl;

	updateTexture();

	{
		const numComponents = 2;
		const type = gl.FLOAT;
		const normalize = false;
		const stride = 0;
		const offset = 0;
		gl.bindBuffer(gl.ARRAY_BUFFER, vsBuffer);
		gl.vertexAttribPointer(
			programInfo.attribLocations.vertexPosition,
			numComponents,
			type,
			normalize,
			stride,
			offset);
		gl.enableVertexAttribArray(
			programInfo.attribLocations.vertexPosition);
	}

	gl.useProgram(programInfo.program);
	gl.activeTexture(gl.TEXTURE0);
	gl.bindTexture(gl.TEXTURE_2D, mapTexture);
	gl.uniform1i(programInfo.uniformLocations.uMapSampler, 0);

	{
		const offset = 0;
		const vertexCount = 4;
		gl.drawArrays(gl.TRIANGLE_STRIP, offset, vertexCount);
	}
}

function updateTexture() {
	const { programInfo, vsBuffer, mapTexture } = state;
	const gl = e2.gl;

	gl.bindTexture(gl.TEXTURE_2D, mapTexture);

	// TODO un-hardode
	const width = 65;
	const height = 36;

	var raw_data = [];

	for (var y = 0; y < height; y++) {
		for (var x = 0; x < width; x++) {
			const v = e2.world.fluidmap.grid[x + y * width];

			var col = [0, 0, 0, 0];

			for (var i = 0; i < v.length; i++) {
				col[3] = 255;

				if (v[i].owner == 0) {
					col[2] = 255;
				} else if (v[i].owner == 1) {
					col[0] = 255;
				}
			}

			raw_data.push(...col);
		}
	}

	const data = Uint8Array.from(raw_data);

	gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, width, height, 0, gl.RGBA, gl.UNSIGNED_BYTE, data);
}

function range(n) {
	return [...Array(n).keys()];
}

