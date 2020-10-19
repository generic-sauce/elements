#version 450

layout(location = 0) out vec4 frag_color;

layout (location = 0) in vec3 uv;
layout (location = 1) in vec3 color;

void main() {
	vec3 c = color;
	frag_color = vec4(c, 1);
}
