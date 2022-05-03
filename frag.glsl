#version 330 core
out vec4 frag_color;

in vec3 o_col;
in vec2 o_uv;

uniform sampler2D texture1;

void main() 
{
    frag_color = texture(texture1, o_uv);
}
