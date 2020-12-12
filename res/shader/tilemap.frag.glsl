#version 450

layout (location = 0) out vec4 frag_color;

layout (set = 0, binding = 0) uniform texture2D tilemap_tex;
layout (set = 0, binding = 1) uniform sampler tilemap_sam;

layout (location = 0) in vec2 uv;

float n21(vec2 s) {
	return fract(9542.276 * sin(dot(vec2(527.831, 699.258), s)));
}

float smooth_n21(vec2 seed) {
	vec2 id = floor(seed);
	float nlb = n21(id);
	float nlt = n21(id + vec2(0, 1));
	float nrb = n21(id + vec2(1, 0));
	float nrt = n21(id + vec2(1, 1));

	vec2 frac = smoothstep(0., 1., fract(seed));
	float nl = mix(nlb, nlt, frac.y);
	float nr = mix(nrb, nrt, frac.y);
	float n = mix(nl, nr, frac.x);
	return n;
}

float round_n21(vec2 seed) {
	seed += 17.63;
	float n0 = smooth_n21(seed);
	float n1 = smooth_n21(seed * 2.);
	float n2 = smooth_n21(seed * 4.);
	float n3 = smooth_n21(seed * 8.);

	return n0 * .1 + n1 * .2 + n2 * .3 + n3 * .4;
}

vec2 tilemap_size() {
	return textureSize(sampler2D(tilemap_tex, tilemap_sam), 0);
}

int tile(vec2 uv) {
	return int(255.9 * texture(sampler2D(tilemap_tex, tilemap_sam), uv).r);
}

float tilef(vec2 uv) {
	return 255. * texture(sampler2D(tilemap_tex, tilemap_sam), uv).r;
}

float ground(vec2 uv) {
	return tile(uv) == 1 ? 1. : 0.;
}

vec3 ground_color(vec2 uv) {
	// tweaks
	const float density = 2.;
	const float grass_scale = .7;
	float grass_thickness = 3.;
	const float dirt_grass_threshold = .2;

	// colors
	vec3 dirt0 = vec3(75, 50, 27) / 255. * 1.1;
	vec3 dirt1 = vec3(229, 187, 128) / 255. * .7;
	vec3 grass0 = vec3(123, 231, 118) / 255. * .8;
	vec3 grass1 = vec3(15, 75, 32) / 255.;

	// consts
	vec2 size = tilemap_size();
	vec2 aspect = vec2(size.x / size.y, 1);
	vec2 px = 1. / size;
	vec2 pxx = vec2(px.x, 0);
	vec2 pxy = vec2(0, px.y);

	// grid cell local uv
	vec2 lv = fract(uv * size);

	float sum = 0.;
	float all = 1.;
	for (float i = 1.; i < grass_thickness + 1.5; ++i) {
		float t = ground(uv + pxy * i);
		all *= t;
		sum += all;
	}

	float grass_portion = 0.;
	grass_portion += lv.y + (grass_thickness - sum);
	grass_portion += (1. - lv.x) * (1. - ground(uv + pxy * sum - pxx));
	grass_portion += lv.x * (1. - ground(uv + pxy * sum + pxx));
	grass_portion *= (grass_scale / grass_thickness);
	grass_portion *= 1. - all;

	float n_dirt = round_n21(uv * aspect * density);
	float n_grass = mix(n_dirt, .0, grass_portion);

	float dirt_color_ratio = floor(n_dirt * 4.) / 4.;
	float grass_color_ratio = floor(n_grass / dirt_grass_threshold * 4.) / 4.;

	vec3 dirt_color = mix(dirt0, dirt1, vec3(dirt_color_ratio));
	vec3 grass_color = mix(grass0, grass1, vec3(grass_color_ratio));

	/* return vec3(grass_portion); */
	return mix(grass_color, dirt_color, step(dirt_grass_threshold, n_grass));
}

void main() {
	vec3 c;
	vec3 wall_color = vec3(.85, .95, .99);

	int t = tile(uv);
	switch (t) {
	case 1: // ground
		c = ground_color(uv + .00001);
		break;
	case 2: // wall player 0
		c = wall_color;
		break;
	case 3: // wall player 1
		c = (1. - wall_color) * .5;
		break;
	case 0: // background
	default:
		{
			discard; // discard until depth test is enabled
			/* vec3 bright_bg = vec3(133, 178, 215) / 255.; */
			/* vec3 dark_bg = vec3(37, 78, 205) / 255.; */
			/* c = mix(dark_bg, bright_bg, vec3(uv.y)); */
		}
	}

	c = pow(c, vec3(2.2));
	frag_color = vec4(c, 1);
}
