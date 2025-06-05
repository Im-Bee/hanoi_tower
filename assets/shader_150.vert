#version 150 core

in vec3 a_pos;


out vec3 v_FragPos;
out vec3 v_Normal;

uniform mat4 u_model_view_proj;

void main() 
{
    gl_Position = u_model_view_proj * vec4(a_pos, 1.0);

    v_FragPos = vec3(gl_Position);
    v_Normal = vec3(transpose(inverse(u_model_view_proj))) * 1.0;
}
