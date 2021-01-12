export default`
precision mediump float;

uniform sampler2D tex;

varying vec2 uv;
varying vec3 color;

void main() {
	vec4 t = texture2D(tex, uv);
	vec3 c = color * t.rgb;
	gl_FragColor = vec4(c, t.a);
}
`
