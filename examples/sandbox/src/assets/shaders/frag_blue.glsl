#version 450

layout(location = 0) in vec3 v_Position;

layout(location = 0) out vec4 color;

void main()
{
    color = vec4(0.2, 0.3, 0.8, 1.0);
}