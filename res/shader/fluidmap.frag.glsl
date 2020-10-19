#version 450

layout (location = 0) out vec4 frag_color;

layout (set = 0, binding = 0) uniform texture2D fluidmap_tex;
layout (set = 0, binding = 1) uniform sampler fluidmap_sam;

layout (location = 0) in vec2 uv;

void main() {
	vec3 c;

	int fluid = int(255.9 * texture(sampler2D(fluidmap_tex, fluidmap_sam), uv).r);
	switch (fluid) {
	case 0:
		c = vec3(0, uv);
		break;
	case 1:
		c = vec3(1, uv);
		break;
	default:
		discard;
	}

	frag_color = vec4(c, 1);
}
