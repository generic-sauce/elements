#version 130

uniform sampler2D input_tex;

in vec2 uv;

out vec4 color;

float n21(vec2 uv) {
	return fract(9232.43 * sin(dot(uv, vec2(123.42, 642.332))));
}

void main() {
	vec4 rgba = texture(input_tex, uv);
	vec3 c = rgba.rgb;
	float n = (rgba.r + rgba.b + rgba.g * 2.) / 4.;
	float a = rgba.a;

	c += n21(uv) * .1;

	color = vec4(c, a);
}
