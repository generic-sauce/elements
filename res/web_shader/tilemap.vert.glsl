attribute vec2 vertex_position;
attribute vec2 vertex_uv;

varying vec2 uv;

void main() {
	uv = vertex_uv;
	gl_Position = vec4(vertex_position, 0, 1);
}
