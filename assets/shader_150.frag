#version 150 core

in vec3 v_FragPos;
in vec3 v_Normal;

out vec4 o_Color;

uniform vec4 t_color;


void main() 
{
    // Constant values
    vec3 lightPos   = vec3(15.0, -20.0,  20.0);
    vec3 lightColor = vec3( 1.0,   0.9,   1.0);
    vec3 viewPos    = vec3( 0.5,  10.5,  21.0);

    float ambientStrength  =  0.1;
    float diffuseStrength  =  1.4;
    float specularStrength = 21. ;
    float shininess        = 20.0;

    // Ambient
    vec3 ambient = ambientStrength * lightColor;

    // Diffuse
    vec3 norm = normalize(v_Normal);
    vec3 lightDir = normalize(lightPos - v_FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diffuseStrength * diff * lightColor;

    // Specular
    vec3 viewDir = normalize(viewPos - v_FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
    vec3 specular = specularStrength * spec * lightColor;

    vec3 result = (ambient + diffuse + specular) * vec3(t_color);
    o_Color = vec4(result, 1.0);
}
