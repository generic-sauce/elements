export default `
precision mediump float;

uniform sampler2D tilemap_tex;

varying vec2 uv;

void main() {
	int tile = int(255.9 * texture2D(tilemap_tex, uv).r);

	vec4 t = texture2D(tex, uv);
	if (t.a == 0.)
		gl_FragColor.a = 1.;
	else
		gl_FragColor.a = gl_FragCoord.z;
}
`
