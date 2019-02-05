attribute vec2 board_position;
attribute vec3 board_center;
attribute vec4 a_color;

uniform vec3 camera_up;
uniform vec3 camera_right;
uniform vec4 worldspace_transform;

varying vec4 v_color;

void main() {
  vec3 worldspace_position = 
      board_center 
    + camera_up    * board_position.x
    + camera_right * board_position.y;

  gl_Position = worldspace_transform * vec4(worldspace_position, 1.0);

  v_color = a_color;
}
