#version 140

in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;

out vec2 my_attr;

uniform mat4 matrix;

void main() {
    my_attr = position;
    v_tex_coords = tex_coords;
    vec2 pos = position;
    gl_Position = matrix * vec4(pos, 0.0, 1.0);
}
