#version 140

in vec2 v_tex_coords;
in vec3 v_normal;
in vec2 v_material;

out vec4 color;

uniform sampler2D tex;

void main() {
    color = texture(tex, v_tex_coords) * vec4(v_material.x / 4., 1.0, 1.0, 1.0);
}
