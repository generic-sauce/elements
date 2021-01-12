export default `
precision mediump float;

uniform sampler2D fluidmap_tex;
uniform vec2 fluidmap_tex_size;
uniform float elapsed_time;

varying vec2 uv;

float smin(float a, float b, float k) {
	float h = clamp(0.5+0.5*(b-a)/k, 0.0, 1.0);
	return mix(b, a, h) - k*h*(1.0-h);
}

void main() {
	vec3 c = vec3(0);
	vec4 z = vec4(0, 0, 0, 0);

	vec2 id = floor(uv * fluidmap_tex_size);
	vec2 lv = fract(uv * fluidmap_tex_size) - .5;
	const float r = 2.;
	float d = r * .5;
	float team = 0.;

	for (float x = -r; x < r+.5; ++x) {
		for (float y = -r; y < r+.5; ++y) {
			vec2 o = vec2(x, y);
			/* vec2 gid = id + o; */
			vec2 guv = uv + o / fluidmap_tex_size;
			vec4 tile = texture2D(fluidmap_tex, guv);
			vec2 glv = lv - o - (tile.xy - .5);
			float team_ = tile.b;
			/* int fluid = int(255.9 * texture2D(fluidmap_tex, guv).r); */
			/* vec2 glv = lv - o; */
			/* float team_ = float(fluid); */
			if (tile.a > .5) {
			/* if (fluid < 2) { */
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

	gl_FragColor = vec4(c, alpha);
}
`
