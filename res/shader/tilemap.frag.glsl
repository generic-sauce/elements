#version 450

layout (location = 0) out vec4 frag_color;

layout (set = 0, binding = 0) uniform texture2D tilemap_tex;
layout (set = 0, binding = 1) uniform sampler tilemap_sam;

layout (location = 0) in vec2 uv;

float n21(ivec2 s) {
	vec2 sf = vec2(s);
	return fract(9542.276 * sin(dot(vec2(527.831, 699.258), sf)));
}

float smooth_n21(ivec2 s) {
	float n0 = n21(s);
	float n1 = n21(s / 2 * 2 + 1000);
	float n2 = n21(s / 4 * 4 + 2000);
	float n3 = n21(s / 8 * 8 + 2000);

	return n0 * .2 + n1 * .3 + n2 * .3 + n3 * .2;
}

vec3 ground_color(vec2 uv) {
	vec2 tilemap_tex_size = textureSize(sampler2D(tilemap_tex, tilemap_sam), 0);

	const vec3 colors[] = vec3[] (
		vec3(69, 48, 31),
		vec3(75, 55, 44),
		vec3(86, 67, 48),
		vec3(95, 78, 60),
		vec3(48, 30, 11),
		vec3(65, 45, 29),
		vec3(88, 63, 48)
	);

	ivec2 uvi = ivec2(uv * tilemap_tex_size);
	int i = int(smooth_n21(uvi) * 7.);
	return colors[i] / 255.;
}

void main() {
	vec3 c;
	vec3 wall_color = vec3(.85, .95, .99);

	int tile = int(255.9 * texture(sampler2D(tilemap_tex, tilemap_sam), uv).r);
	switch (tile) {
	case 1: // ground
		c = ground_color(uv);
		break;
	case 2: // wall player 0
		c = wall_color;
		break;
	case 3: // wall player 1
		c = (1. - wall_color) * .5;
		break;
	case 0:
	default:
		discard;
	}

	c = pow(c, vec3(2.2));
	frag_color = vec4(c, 1);
}
