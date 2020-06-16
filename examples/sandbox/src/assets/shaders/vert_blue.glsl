#version 450

layout(location = 0) in vec3 a_Position;

// uniform mat4 u_ViewProjection;

layout(location = 0) out vec3 v_Position;

void main()
{
    v_Position = a_Position;
    // gl_Position = u_ViewProjection * vec4(a_Position, 1.0);	
    gl_Position = vec4(a_Position, 1.0);	
}