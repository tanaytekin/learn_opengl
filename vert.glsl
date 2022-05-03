#version 330 core

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec3 a_col;
layout(location = 2) in vec2 a_uv;

out vec3 o_col;
out vec2 o_uv;

void main() 
{
    o_col = a_col;
    o_uv= a_uv;
    gl_Position = vec4(a_pos.xyz, 1.0);
}
