#version 450

layout(location = 0) in vec2 uv;
layout(set = 0, binding = 0) uniform texture2D tex;
layout(set = 0, binding = 1) uniform sampler sam;

layout(location = 0) out vec4 color;

void main() {
	vec4 c = texture(sampler2D(tex, sam), uv);
	color = vec4(vec3(dot(c.rgb, vec3(1./3.))), 1);
}
