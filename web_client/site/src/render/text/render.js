import { state } from "./init"
import { state as triangles_state } from "../triangles/init"
import { bind_vao } from "../triangles/render"

export function render(text) {
	gl.useProgram(triangles_state.program)
	gl.bindBuffer(gl.ARRAY_BUFFER, state.buffer)

	bind_vao()

	update_texture(text.left_bot, text.scale, text.color, text.string)

	{
		const offset = 0
		const count = 4
		gl.drawArrays(gl.TRIANGLE_STRIP, offset, count)
	}
}

function update_texture(left_bot, scale, color, text) {
	const text_canvas = document.createElement("canvas")
	const ctx = text_canvas.getContext("2d")

	// TODO: this is an educated guess
	const font_size = scale * canvas.height * 2
	// console.log(scale); alert()
	const font = `${font_size}px elements_font`
	ctx.font = font

	const box = ctx.measureText(text)
	const le = box.actualBoundingBoxLeft
	const ri = box.actualBoundingBoxRight
	const to = box.actualBoundingBoxAscent
	const bo = box.actualBoundingBoxDescent

	const left  = le / -canvas.width
	const right = ri /  canvas.width
	const top   = to /  canvas.height
	const bot   = bo / -canvas.height

	const x = left_bot[0]
	const y = left_bot[1]
	const r = color.r
	const g = color.g
	const b = color.b
	const a = color.a

	// TODO: vertex positions also incorrect
	const vertices = [
		x + right, y + bot, 1.0, 0.0, r, g, b, a,
		x + right, y + top, 1.0, 1.0, r, g, b, a,
		x + left,  y + bot, 0.0, 0.0, r, g, b, a,
		x + left,  y + top, 0.0, 1.0, r, g, b, a,
	]

	gl.bindBuffer(gl.ARRAY_BUFFER, state.buffer)
	gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW)

	text_canvas.width = (ri - le)
	text_canvas.height = (to + bo)

	ctx.fillStyle = "transparent"
	ctx.fillRect(0, 0, text_canvas.width, text_canvas.height)

	ctx.font = font
	ctx.textAlign = "left"
	ctx.textBaseline = "alphabetic"
	ctx.fillStyle = "white"
	ctx.fillText(text, 0, to)

	gl.bindTexture(gl.TEXTURE_2D, state.texture)
	{
		const level = 0
		const src_type = gl.UNSIGNED_BYTE
		const src_format = gl.RGBA
		const dst_format = gl.RGBA
		gl.texImage2D(gl.TEXTURE_2D, level, dst_format, src_format, src_type, text_canvas)
	}
}
