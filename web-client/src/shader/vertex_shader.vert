#version 400

in vec3 position;

uniform mat4 object_transform;
uniform vec4 u_color;

out vec4 f_color;
out float distance; 

void main() {
    f_color = u_color;
    distance = length(position);
    gl_Position = object_transform * vec4(position, 1.0);
}

