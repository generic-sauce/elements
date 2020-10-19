#version 450

layout (location = 0) out vec4 frag_color;

layout (set = 0, binding = 0) uniform texture2D tilemap_tex;
layout (set = 0, binding = 1) uniform sampler tilemap_sam;

layout (location = 0) in vec2 uv;

void main() {
	vec3 c;

	vec3 ground_color = vec3(
		109. / 255.,
		 72. / 255.,
		 35. / 255.
	);
	vec3 wall_color = vec3(.85, .95, .99);

	int tile = int(255.9 * texture(sampler2D(tilemap_tex, tilemap_sam), uv).r);
	switch (tile) {
	case 1: // ground
		c = vec3(.12, .06, .05);
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

	frag_color = vec4(c, 1);
}
