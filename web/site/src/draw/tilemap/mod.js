import { init, state } from "./init.js";

export { init, draw }

function draw() {
	const { gl, programInfo, vsBuffer, mapTexture } = state;

	updateTexture();

	{
		const numComponents = 2;
		const type = gl.FLOAT; // TODO is it?
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
		gl.drawArrays(gl.TRIANGLES, offset, vertexCount);
	}
}

function updateTexture() {
	const { gl, programInfo, vsBuffer, mapTexture } = state;

	gl.bindTexture(gl.TEXTURE_2D, mapTexture);

	// TODO un-hardode
	const width = 128;
	const height = 72;

	const raw_data = range(width * height).map(_ => [0, 255, 0, 255]);
	const data = null;  // Uint8Array.from(raw_data);

	gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, width, height, 0, gl.RGBA, gl.UNSIGNED_BYTE, data);
}

function range(n) {
	return [...Array(n).keys()];
}

