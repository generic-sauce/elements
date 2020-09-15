#version 130
/* precision mediump float; */

uniform sampler2D tilemap_tex;

in vec2 uv;

out vec4 color;

void main() {
	vec4 tile = texture(tilemap_tex, uv);
	float ground = tile.r;
	float team = tile.g;
	float ratio = tile.b;
	float alpha = ground;

	vec3 ground_color = vec3(.25, .1, .1);
	vec3 wall_color = vec3(.85, .95, .99);
	vec3 team_color = mix(wall_color, 1.-wall_color, team);
	vec3 c = mix(ground_color, team_color, ratio);
	color = vec4(c, alpha);
}
