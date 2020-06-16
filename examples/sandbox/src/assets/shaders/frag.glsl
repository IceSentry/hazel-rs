#version 450

layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec4 v_Color;

layout(location = 0) out vec4 color;

void main()
{
    color = vec4(v_Position * 0.5 + 0.5, 1.0);
    color = v_Color;
}