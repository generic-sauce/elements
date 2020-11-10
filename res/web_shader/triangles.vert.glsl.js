export default`
attribute vec3 vertex_position;
attribute vec2 vertex_uv;
attribute vec3 vertex_color;

varying vec2 uv;
varying vec3 color;

void main() {
	uv = vertex_uv;
	uv.y = 1. - uv.y;
	color = vertex_color;
	gl_Position = vec4(vertex_position, 1);
}
`
