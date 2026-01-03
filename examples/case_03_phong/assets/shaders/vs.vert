out vec3 frag_pos_in_world;
out vec3 normal_in_world;
out vec2 tex_coord;

void main() {
  // 点在世界坐标中的位置
  frag_pos_in_world = vec3(MODEL_MATRIX * vec4(POSITION, 1.0));
  // 法线在世界坐标中的向量
  normal_in_world = NORMAL_MATRIX * NORMAL;
  // 纹理坐标
  tex_coord = TEXCOORD;
  // 屏幕位置
  gl_Position = PROJECTION_MATRIX * VIEW_MATRIX * vec4(frag_pos_in_world, 1.0);
}