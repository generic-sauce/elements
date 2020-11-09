export default`
attribute vec2 vertex_position;
attribute vec2 vertex_uv;
attribute vec3 vertex_color;

varying vec2 uv;
varying vec3 color;

void main() {
	uv = vertex_uv;
	color = vertex_color;
	gl_Position = vec4(vertex_position, 0, 1);
}
`
