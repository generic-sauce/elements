export function load(filepaths, callback) {
	let textures = []

	// TODO load images, then call code underneaht, then call callback

	for (let i = 0; i < filepaths.length; ++i) {
		let texture = gl.createTexture()
		gl.bindTexture(gl.TEXTURE_2D, texture)

		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR)
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR)

		// gl.texImage2D(gl.TEXTURE_2D); // TODO

		textures.push(texture)
	}

	return textures
}
