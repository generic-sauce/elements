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
	float alpha = tile.a;

	vec3 ground_color = mix(vec3(.45, .62, .22), vec3(.2, .1, .1), ground);
	vec2 wall_color = vec2(.8, .2);
	vec3 team_color = mix(wall_color.yyx, wall_color.xyy, team);
	vec3 c = mix(ground_color, team_color, ratio);
	color = vec4(c, alpha);
}
