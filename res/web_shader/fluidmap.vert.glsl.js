export default `
attribute vec2 vertex_position;
attribute vec2 vertex_uv;

varying vec2 uv;

void main() {
	uv = vertex_uv;

	// ensure that floor(vertex_position) works on different scales
	gl_Position = vec4(vertex_position.xy + .001, 0, 1);
}
`
