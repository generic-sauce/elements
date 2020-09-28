#version 450

out gl_PerVertex {
	vec4 gl_Position;
};

layout(location = 0) out vec2 uv;

const vec2 positions[6] = vec2[6](
    // Upper left triangle
    vec2(-1.0, -1.0),
    vec2(1.0, -1.0),
    vec2(-1.0, 1.0),

    // Lower right triangle
    vec2(-1.0, 1.0),
    vec2(1.0, -1.0),
    vec2(1.0, 1.0)
);

const vec2 uvs[6] = vec2[6](
    // Upper left triangle
    vec2(0.0, 0.0),
    vec2(1.0, 0.0),
    vec2(0.0, 1.0),

    // Lower right triangle
    vec2(0.0, 1.0),
    vec2(1.0, 0.0),
    vec2(1.0, 1.0)
);

void main() {
	uv = uvs[gl_VertexIndex];
	gl_Position = vec4(positions[gl_VertexIndex], 0, 1);
}
