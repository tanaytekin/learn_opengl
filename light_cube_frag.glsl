
#version 330 core
out vec4 frag_color;


in vec3 o_normal;
in vec3 o_frag_pos;

uniform vec3 light_color;


void main() 
{
    frag_color = vec4(light_color, 1.0);
}
