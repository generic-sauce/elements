/* #version 300 es */
/* precision mediump float; */

uniform sampler2D fluid_tex;

void main() {
	gl_FragColor = vec4(texture(fluid_tex, gl_TexCoord[0].xy).rgb, .5);
}
