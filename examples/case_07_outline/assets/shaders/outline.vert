layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 texcoord;

out vec3 frag_position; // 片段位置
out vec3 frag_normal;   // 片段法线

void main() {
  vec3 scale_position = position * (1.1);

  // 将顶点位置从模型空间转换到世界空间
  frag_position = vec3(MODEL_MATRIX * vec4(scale_position, 1.0));

  // 将法线从模型空间转换到世界空间
  frag_normal = normalize(NORMAL_MATRIX * normal);

  // 计算最终的裁剪空间位置
  gl_Position = PVM_MATRIX * vec4(scale_position, 1.0);
}