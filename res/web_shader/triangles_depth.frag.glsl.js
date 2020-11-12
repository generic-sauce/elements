export default`
precision mediump float;

uniform sampler2D tex;

varying vec2 uv;

void main() {
	// float alpha = texture2D(tex, uv).a;
	// float factor = step(.5, alpha);
	// float depth = 1. - factor * (1. - gl_FragCoord.z);
  //
	// gl_FragColor.a = depth;

	float alpha = texture2D(tex, uv).a;
	if (alpha < .5)
		gl_FragColor.a = 1.;
	else
		gl_FragColor.a = gl_FragCoord.z;
}
`
