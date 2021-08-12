#version 450

layout (location = 0) out vec4 frag_color;

layout (set = 0, binding = 0) uniform texture2D tilemap_tex;
layout (set = 0, binding = 1) uniform sampler tilemap_sam;

layout (location = 0) in vec2 uv;

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
	seed += 17.63;
	float n0 = smooth_n21(seed);
	float n1 = smooth_n21(seed * 2.);
	float n2 = smooth_n21(seed * 4.);
	float n3 = smooth_n21(seed * 8.);

	return n0 * .1 + n1 * .2 + n2 * .3 + n3 * .4;
}

vec2 tilemap_size() {
	return textureSize(sampler2D(tilemap_tex, tilemap_sam), 0);
}

int tile(vec2 uv) {
	return int(255.9 * texture(sampler2D(tilemap_tex, tilemap_sam), uv).r);
}

float tilef(vec2 uv) {
	return 255. * texture(sampler2D(tilemap_tex, tilemap_sam), uv).r;
}

float ground(vec2 uv) {
	return tile(uv) == 1 ? 1. : 0.;
}

float xor(float a, float b) {
	return 1. - abs(1. - a - b);
}

vec4 ground_color(vec2 uv) {
	return vec4(.47, .43, .36, 1);
}

void main() {
	vec4 c;
	vec3 wall_color = vec3(.85, .95, .99);

	int t = tile(uv);
	switch (t) {
	case 1: // ground
		c = ground_color(uv + .00001);
		break;
	case 2: // wall player 0
		c = vec4(wall_color, 1);
		break;
	case 3: // wall player 1
		c = vec4((1. - wall_color) * .5, 1);
		break;
	case 0: // background
	default:
		c.a = 0.;
	}

	if (c.a < .5)
		discard;

	c.rgb = pow(c.rgb, vec3(2.2));
	frag_color = vec4(c.rgb, 1);
}
