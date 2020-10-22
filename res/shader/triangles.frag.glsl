#version 450

layout(location = 0) out vec4 frag_color;

layout (set = 0, binding = 0) uniform texture2D tex;
layout (set = 0, binding = 1) uniform sampler sam;

layout (location = 0) in vec2 uv;
layout (location = 1) in vec3 color;

void main() {
	vec4 t = texture(sampler2D(tex, sam), uv);
	if (t.a < .5)
		discard;

	vec3 c = color;
	frag_color = vec4(c * t.rgb, 1);
}
