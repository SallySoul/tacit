attribute vec3 position;

uniform mat4 object_transform;

void main() {
    gl_Position = object_transform * vec4(position, 1.0);
}

