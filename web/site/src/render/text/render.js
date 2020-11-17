import { state } from "./init"

export function render(draw) {
	gl.useProgram(state.program)
	gl.bindBuffer(gl.ARRAY_BUFFER, state.buffer)

	{ // vertex_position
		const count = 2
		const type = gl.FLOAT
		const normalize = false
		const stride = 7 * 4
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
		const stride = 7 * 4
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
		const count = 3
		const type = gl.FLOAT
		const normalize = false
		const stride = 7 * 4
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

	for (let i = 0; i < draw.texts.length; ++i) {
		const text = draw.texts[i]
		update_texture(text.left_bot, text.string)

		const offset = 0
		const count = 4
		gl.drawArrays(gl.TRIANGLE_STRIP, offset, count)
	}
}

function update_texture(left_bot, text) {
	const text_canvas = document.createElement("canvas")
	const ctx = text_canvas.getContext("2d")

	const font_size = 72
	const font = `${font_size}px elements_font`
	ctx.font = font

	const box = ctx.measureText(text)
	const left = box.actualBoundingBoxLeft   / -canvas.width
	const right = box.actualBoundingBoxRight / canvas.width
	const top = box.actualBoundingBoxAscent  / canvas.height
	const bot = box.actualBoundingBoxDescent / -canvas.height

	const x = left_bot[0]
	const y = left_bot[1]
	const r = 1.0
	const g = 1.0
	const b = 1.0
	const vertices = [
		x + right, y + bot, 1.0, 0.0, r, g, b,
		x + right, y + top, 1.0, 1.0, r, g, b,
		x + left,  y + bot, 0.0, 0.0, r, g, b,
		x + left,  y + top, 0.0, 1.0, r, g, b,
	]

	gl.bindBuffer(gl.ARRAY_BUFFER, state.buffer)
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW)

	const width = (right - left) * canvas.width
	const height = (top - bot) * canvas.height

	text_canvas.width = width
	text_canvas.height = height
	text_canvas.style.width = width + "px"
	text_canvas.style.height = height + "px"

	ctx.font = font
	ctx.textAlign = "left"
	ctx.textBaseline = "bottom"

	ctx.fillStyle = "transparent"
	ctx.fillRect(0, 0, text_canvas.width, text_canvas.height)

	ctx.fillStyle = "white"
	ctx.fillText(text, 0, font_size)

	gl.bindTexture(gl.TEXTURE_2D, state.texture)
	{
		const level = 0
		const src_type = gl.UNSIGNED_BYTE
		const src_format = gl.RGBA
		const dst_format = gl.RGBA
		gl.texImage2D(gl.TEXTURE_2D, level, dst_format, src_format, src_type, text_canvas)
	}
}
