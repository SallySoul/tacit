attribute vec3 position;
attribute vec4 a_color;

varying vec4 v_color;

void main() {
  gl_Position = vec4(position, 1.0);
  v_color = a_color;
}

