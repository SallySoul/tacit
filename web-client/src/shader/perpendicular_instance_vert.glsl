attribute instance_vertex_postion vec2;
attribute origin_position vec3;

uniform camera_postion vec3;
uniform world_to_clipspace_transform mat4;

void main() {
  vec3 camera_normal = camera_postion - origin_position;
  vec3 instance_normal =  
  

  gl_Position = object_transform * vec4(position, 1.0);
}
