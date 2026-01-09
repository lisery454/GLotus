out vec3 frag_position; // 片段位置
out vec3 frag_normal;   // 片段法线

void main() {
  // 将顶点位置从模型空间转换到世界空间
  frag_position = vec3(MODEL_MATRIX * vec4(POSITION, 1.0));

  // 将法线从模型空间转换到世界空间
  frag_normal = normalize(NORMAL_MATRIX * NORMAL);

  // 计算最终的裁剪空间位置
  gl_Position = PVM_MATRIX * vec4(POSITION, 1.0);
}