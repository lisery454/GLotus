// 输入
in vec3 frag_position; // 片段位置
in vec3 frag_normal;   // 片段法线

// 输出颜色
out vec4 frag_color;

float LinearizeDepth(float glFragCoordZ) {
  float near = CAMERA.near_plane;
  float far = CAMERA.far_plane;
  float z = glFragCoordZ * 2.0 - 1.0;
  return (2.0 * near * far) / (far + near - z * (far - near)) / far;
}

void main() { frag_color = vec4(vec3(LinearizeDepth(gl_FragCoord.z)), 1.0); }