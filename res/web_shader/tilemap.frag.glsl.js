export default `
precision mediump float;

uniform sampler2D tilemap_tex;
uniform vec2 tilemap_tex_size;

varying vec2 uv;

float n21(vec2 s) {
	return fract(9542.276 * sin(dot(vec2(527.831, 699.258), s)));
}

float smooth_n21(vec2 seed) {
	vec2 id = floor(seed);
	float nlb = n21(id);
	float nlt = n21(id + vec2(0, 1));
	float nrb = n21(id + vec2(1, 0));
	float nrt = n21(id + vec2(1, 1));

	vec2 frac = smoothstep(0., 1., fract(seed));
	float nl = mix(nlb, nlt, frac.y);
	float nr = mix(nrb, nrt, frac.y);
	float n = mix(nl, nr, frac.x);
	return n;
}

float round_n21(vec2 seed) {
	float n0 = smooth_n21(seed);
	float n1 = smooth_n21(seed * 2.);
	float n2 = smooth_n21(seed * 4.);
	float n3 = smooth_n21(seed * 8.);

	return n0 * .1 + n1 * .2 + n2 * .3 + n3 * .4;
}

vec3 color_by_index(int index) {
	if (index == 0)
		return vec3(20,36,21);
	if (index == 1)
		return vec3(21,29,15);
	if (index == 2)
		return vec3(24,43,29);
	if (index == 3)
		return vec3(35,49,27);
	return vec3(28,48,27);
}

vec3 ground_color(vec2 uv) {
	const int count = 5;

	float h = round_n21(uv * tilemap_tex_size / 16.);
	h = sign(h - .5) * pow(abs(h * 2. - 1.), .8) * .5 + .5;
	int i = int(float(count) * h);
	vec3 c = color_by_index(i);

	return c / 255.;
}

void main() {
	vec3 c;
	vec3 wall_color = vec3(.85, .95, .99);

	int tile = int(255.9 * texture2D(tilemap_tex, uv).r);
	if (tile == 1) // ground
		c = ground_color(uv);
	else if (tile == 2) // wall player 0
		c = wall_color;
	else if (tile == 3) // wall player 1
		c = (1. - wall_color) * .5;
	else // background
		discard;

	gl_FragColor = vec4(c, 1);
}
`
