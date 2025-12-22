layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 texcoord;

out vec3 frag_pos_in_world;
out vec3 normal_in_world;
out vec2 tex_coord;

void main() {
  // 点在世界坐标中的位置
  frag_pos_in_world = vec3(g_model_matrix * vec4(position, 1.0));
  // 法线在世界坐标中的向量
  normal_in_world = g_normal_matrix * normal;
  // 纹理坐标
  tex_coord = vec2(texcoord.x, texcoord.y);
  // 屏幕位置
  gl_Position = g_projection_matrix * g_view_matrix * vec4(frag_pos_in_world, 1.0);
}