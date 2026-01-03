// 输入
in vec3 frag_position; // 片段位置
in vec3 frag_normal;   // 片段法线

// 输出颜色
out vec4 frag_color;

void main() {
  vec3 normal = normalize(frag_normal);
  vec3 color = normal * 0.5 + vec3(0.5, 0.5, 0.5);
  frag_color = vec4(color, 1.0);
}