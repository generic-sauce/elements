#version 450

layout (location = 0) out vec4 frag_color;

layout (set = 0, binding = 0) uniform texture2D tilemap_tex;
layout (set = 0, binding = 1) uniform sampler tilemap_sam;

layout (location = 0) in vec2 uv;

void main() {
	vec3 c;

	int tile = int(255.9 * texture(sampler2D(tilemap_tex, tilemap_sam), uv).r);
	switch (tile) {
	case 1:
		c = vec3(uv, 0);
		break;
	case 2:
		c = vec3(1. - uv, 1);
		break;
	case 3:
		c = vec3(1. - uv, 0);
		break;
	case 0:
	default:
		discard;
	}

	frag_color = vec4(c, 1);
}
