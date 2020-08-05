/* #version 300 es */
/* precision mediump float; */

varying vec2 uv;

void main()
{
		// transform the vertex position
    gl_Position = gl_ModelViewProjectionMatrix * gl_Vertex;

    // transform the texture coordinates
    uv = (gl_TextureMatrix[0] * gl_MultiTexCoord0).xy;

    // forward the vertex color
    gl_FrontColor = gl_Color;
}
