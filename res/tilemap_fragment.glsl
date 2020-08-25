#version 130
/* precision mediump float; */

uniform vec2 tilemap_tex_size;
uniform sampler2D tilemap_tex;

in vec2 uv;

out vec4 color;

float sdf_square(vec2 uv) {
	uv = abs(uv);
	return max(uv.x, uv.y);
}

float sdf_tilemap(vec2 uv) {
	vec2 ps = 1. / tilemap_tex_size;
	float d = 1.;
	vec2 id = floor(uv * tilemap_tex_size);
	vec2 lv = fract(uv * tilemap_tex_size) - .5;

	for (int i = 0; i < 4; ++i) {
		int s1 = i / 2;
		int s2 = i % 2;
		vec2 oid = vec2(s1 * (s2 * 2 - 1), (1 - s1) * (s2 * 2 - 1));
		vec2 otx = oid * ps;
		vec2 tx = uv + otx;
		vec4 tile = texture(tilemap_tex, tx);

		float ground = tile.r;
		float team = tile.g;
		float ratio = tile.b;
		float alpha = ground;

		if (alpha < .5) {
			vec2 glv = lv - oid;
			d = min(d, sdf_square(glv) - .5);
		}
	}

	for (int i = 0; i < 4; ++i) {
		vec2 oid = vec2((i % 2) * 2 - 1, ((i + (i / 2)) % 2) * 2 - 1);
		vec2 otx = oid * ps;
		vec2 tx = uv + otx;
		vec4 tile = texture(tilemap_tex, tx);

		float ground = tile.r;
		float team = tile.g;
		float ratio = tile.b;
		float alpha = ground;

		if (alpha < .5) {
			vec2 glv = lv - oid;
			d = min(d, sdf_square(glv) - .5);
		}
	}

	return d;
}

void main() {
	vec4 tile = texture(tilemap_tex, uv);
	float ground = tile.r;
	float team = tile.g;
	float ratio = tile.b;
	float alpha = ground;

	float d = sdf_tilemap(uv);
	vec3 ground_color = vec3(.2, .1, d);
	vec3 wall_color = vec3(.85, .95, .99);
	vec3 team_color = mix(wall_color, 1.-wall_color, team);
	vec3 c = mix(ground_color, team_color, ratio);
	color = vec4(c, alpha);
}
