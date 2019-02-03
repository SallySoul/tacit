attribute vec3 postion;
attribute vec4 a_color;

varying vec4 v_color;

void main() {
  gl_Position = vec4(postion, 1.0);
  v_color = a_color;
}

