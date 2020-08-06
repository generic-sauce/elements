#version 130
/* precision mediump float; */

uniform sampler2D tilemap_tex;

in vec2 uv;

out vec4 color;

void main() {
	vec4 tile = texture(tilemap_tex, vec2(0, 1) - vec2(0, uv.y) + vec2(uv.x, 0));
	float ground = tile.r;
	float team = tile.g;
	float ratio = tile.b;
	float alpha = tile.a;

	vec3 ground_color = mix(vec3(.45, .62, .22), vec3(.2, .1, .1), ground);
	vec3 team_color = mix(vec3(1, 0, 0), vec3(0, 0, 1), team);
	vec3 c = mix(ground_color, team_color, ratio);
	color = vec4(c, alpha);
}
