attribute vec2 board_position;
attribute vec3 board_center;

uniform vec3 camera_up;
uniform vec3 camera_right;
uniform vec4 worldspace_transform;

void main() {
  vec3 worldspace_position = 
      board_center 
    + camera_up    * board_position.x
    + camera_right * board_position.y;

  gl_Position = worldspace_transform * vec4(worldspace_position, 1.0);
}
