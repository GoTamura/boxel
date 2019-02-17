#version 140

in uvec3 position;
in vec2 tex_coords;
in vec3 normal;
in vec2 material;

out vec2 v_tex_coords;
out vec3 v_normal;
out vec2 v_material;

uniform mat4 persp_matrix;
uniform mat4 view_matrix;

void main() {
    v_tex_coords = tex_coords;
    v_normal = normal;
    v_material = material;
    gl_Position = persp_matrix * view_matrix * vec4(position * 0.5, 1.0);
}
