#version 330 core

layout(location = 0) in vec3 a_pos;
layout(location = 1) in vec2 a_uv;

out vec2 o_uv;


uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() 
{
    o_uv= a_uv;
    gl_Position = projection * view * model * vec4(a_pos.xyz, 1.0);
}
