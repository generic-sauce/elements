#version 130

uniform float elapsed_time;
uniform vec2 fluid_tex_size;
uniform sampler2D fluid_tex;

in vec2 uv;

out vec4 color;

float smin(float a, float b, float k) {
	float h = clamp(0.5+0.5*(b-a)/k, 0.0, 1.0);
	return mix(b, a, h) - k*h*(1.0-h);
}

void main() {
	vec3 c = vec3(0);
	vec4 z = vec4(0, 0, 0, 0);

	vec2 id = floor(uv * fluid_tex_size);
	vec2 lv = fract(uv * fluid_tex_size) - .5;
	float r = 2.;
	float d = r * .5;
	float team = 0.;

	bool nothing =
		// (texture(fluid_tex, uv + vec2(-2, -2) / fluid_tex_size) == z) &&
		// (texture(fluid_tex, uv + vec2(-2, -1) / fluid_tex_size) == z) &&
		// (texture(fluid_tex, uv + vec2(-2, 0) / fluid_tex_size) == z) &&
		// (texture(fluid_tex, uv + vec2(-2, 1) / fluid_tex_size) == z) &&
		// (texture(fluid_tex, uv + vec2(-2, 2) / fluid_tex_size) == z) &&

		(texture(fluid_tex, uv + vec2(-1, -2) / fluid_tex_size) == z) &&
		(texture(fluid_tex, uv + vec2(-1, -1) / fluid_tex_size) == z) &&
		(texture(fluid_tex, uv + vec2(-1, 0) / fluid_tex_size) == z) &&
		(texture(fluid_tex, uv + vec2(-1, 1) / fluid_tex_size) == z) &&
		(texture(fluid_tex, uv + vec2(-1, 2) / fluid_tex_size) == z) &&

		(texture(fluid_tex, uv + vec2(0, -2) / fluid_tex_size) == z) &&
		(texture(fluid_tex, uv + vec2(0, -1) / fluid_tex_size) == z) &&
		(texture(fluid_tex, uv + vec2(0, 0) / fluid_tex_size) == z) &&
		(texture(fluid_tex, uv + vec2(0, 1) / fluid_tex_size) == z) &&
		(texture(fluid_tex, uv + vec2(0, 2) / fluid_tex_size) == z) &&

		(texture(fluid_tex, uv + vec2(1, -2) / fluid_tex_size) == z) &&
		(texture(fluid_tex, uv + vec2(1, -1) / fluid_tex_size) == z) &&
		(texture(fluid_tex, uv + vec2(1, 0) / fluid_tex_size) == z) &&
		(texture(fluid_tex, uv + vec2(1, 1) / fluid_tex_size) == z) &&
		(texture(fluid_tex, uv + vec2(1, 2) / fluid_tex_size) == z);

		// (texture(fluid_tex, uv + vec2(2, -2) / fluid_tex_size) == z) &&
		// (texture(fluid_tex, uv + vec2(2, -1) / fluid_tex_size) == z) &&
		// (texture(fluid_tex, uv + vec2(2, 0) / fluid_tex_size) == z) &&
		// (texture(fluid_tex, uv + vec2(2, 1) / fluid_tex_size) == z) &&
		// (texture(fluid_tex, uv + vec2(2, 2) / fluid_tex_size) == z);

	if (nothing) {
		color = z;
		return;
	}

	for (float x = -r; x < r+.5; ++x) {
		for (float y = -r; y < r+.5; ++y) {
			vec2 o = vec2(x, y);
			/* vec2 gid = id + o; */
			vec2 guv = uv + o / fluid_tex_size;
			vec4 tile = texture(fluid_tex, guv);
			vec2 glv = lv - o - (tile.xy - .5);
			float team_ = tile.b;
			if (tile.a > .5) {
				float l = length(glv);
				team += (team_ - .5) * max(0., r - l);
				d = smin(d, l, r);
			}
		}
	}

	team = clamp(team * .5, -1., 1.) * .5 + .5;
	float team0 = pow(team, 3.);
	float team1 = pow(1. - team, 3.);
	float teams = pow(team0 + team1, 5.) * .5;

	float t = sin((uv.y + uv.x * -.7) * 50.) + elapsed_time;
	float wave = smoothstep(1., 0., d) * abs(sin(d - t));
	vec3 teams3 = vec3(team0, team1, teams);
	c += vec3(team0, teams, team1) * (2. - wave);

	float alpha = smoothstep(r/4., r/5., d);
	color = vec4(c, alpha);
}
