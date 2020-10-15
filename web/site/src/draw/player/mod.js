import { init, state } from "./init.js";
import { player_rect } from "./math.js";

export { init, draw }

function draw() {
	const { programInfo, buffers, texture } = state;
	const gl = e2.gl;

	gl.bindBuffer(gl.ARRAY_BUFFER, buffers.position);

	const r0 = player_rect(0);
	const r1 = player_rect(1);
	const positions = [r0.slice(0, 6), r0.slice(2,8), r1.slice(0, 6), r1.slice(2,8)].flat();

	gl.bufferData(gl.ARRAY_BUFFER,
								new Float32Array(positions),
								gl.STATIC_DRAW);

	{
    const numComponents = 2;
    const type = gl.FLOAT;
    const normalize = false;
    const stride = 0;
    const offset = 0;
    gl.bindBuffer(gl.ARRAY_BUFFER, buffers.position);
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

  {
    const numComponents = 2;
    const type = gl.FLOAT;
    const normalize = false;
    const stride = 0;
    const offset = 0;
    gl.bindBuffer(gl.ARRAY_BUFFER, buffers.textureCoord);
    gl.vertexAttribPointer(
        programInfo.attribLocations.textureCoord,
        numComponents,
        type,
        normalize,
        stride,
        offset);
    gl.enableVertexAttribArray(
        programInfo.attribLocations.textureCoord);
  }

	gl.useProgram(programInfo.program);
	gl.activeTexture(gl.TEXTURE0);
	gl.bindTexture(gl.TEXTURE_2D, texture);
	gl.uniform1i(programInfo.uniformLocations.uSampler, 0);

	{
		const offset = 0;
		const vertexCount = 12;
		gl.drawArrays(gl.TRIANGLES, offset, vertexCount);
	}
}
