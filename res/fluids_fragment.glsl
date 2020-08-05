#version 130
/* precision mediump float; */

uniform vec2 fluid_tex_size;
uniform sampler2D fluid_tex;

in vec2 uv;

out vec4 gl_FragColor;

float n21(vec2 uv) {
	return fract(9232.43 * sin(dot(uv, vec2(123.42, 642.332))));
}

// polynomial smooth min (k = 0.1);
float smin( float a, float b, float k )
{
    float h = clamp( 0.5+0.5*(b-a)/k, 0.0, 1.0 );
    return mix( b, a, h ) - k*h*(1.0-h);
}

void main() {
	vec3 c = vec3(0);

	vec2 id = floor(uv * fluid_tex_size);
	vec2 lv = fract(uv * fluid_tex_size) - .5;
	float d = 1.;

	float r = 2.;
	for (float x = -r; x < r+1; ++x) {
		for (float y = -r; y < r+1; ++y) {
			vec2 o = vec2(x, y);
			vec2 gid = id + o;
			vec2 guv = gid / fluid_tex_size;
			vec4 tile = texture(fluid_tex, guv);
			vec2 glv = lv - o + (tile.xy - .5);
			if (tile.a > 0.) {
				float l = length(glv);
				d = smin(d, l, r);
			}
			
		}
	}

	/* c += n21(i); */
	c += smoothstep(.5, .0, d);

	gl_FragColor = vec4(c, .1);
}
