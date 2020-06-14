#version 450

// layout(location = 0) in vec3 a_Position;
// out vec3 v_Position;

// void main()
// {
//     v_Position = a_Position;
//     gl_Position = vec4(a_Position, 1.0);	
// }

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_color;

layout(location=0) out vec3 v_color;

void main() {
    v_color = a_color;
    gl_Position = vec4(a_position, 1.0);
}