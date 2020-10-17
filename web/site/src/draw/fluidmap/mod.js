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

	const data = e2.render_world.fluidmap_data;

	const width =  e2.render_world.fluidmap_size[0];
	const height =  e2.render_world.fluidmap_size[1];

	gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, width, height, 0, gl.RGBA, gl.UNSIGNED_BYTE, data);
}

function range(n) {
	return [...Array(n).keys()];
}

