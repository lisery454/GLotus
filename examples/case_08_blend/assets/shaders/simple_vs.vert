layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 texcoord;

out vec3 frag_position; // 片段位置
out vec3 frag_normal;   // 片段法线
out vec2 TexCoord;

void main() {
  // 将顶点位置从模型空间转换到世界空间
  frag_position = vec3(g_model_matrix * vec4(position, 1.0));

  // 将法线从模型空间转换到世界空间
  frag_normal = normalize(g_normal_matrix * normal);

  TexCoord = vec2(texcoord.x, texcoord.y);
  // 计算最终的裁剪空间位置
  gl_Position = g_projection_matrix * g_view_matrix * g_model_matrix *
                vec4(position, 1.0);
}
