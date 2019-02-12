#version 140

in uvec3 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

out vec2 my_attr;

uniform mat4 persp_matrix;
uniform mat4 view_matrix;

void main() {
    my_attr = vec2(0.0, 0.0);
    v_tex_coords = tex_coords;
    vec3 pos = vec3(position) + vec3(0,0, -20);
    pos /= 20.;
    gl_Position = persp_matrix * view_matrix * vec4(position * 0.05, 1.0);
}
