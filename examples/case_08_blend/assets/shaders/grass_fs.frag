// 输入
in vec3 frag_position; // 片段位置
in vec3 frag_normal;   // 片段法线
in vec2 TexCoord;
uniform sampler2D texture1;

// 输出颜色
out vec4 frag_color;

void main() {
  vec4 texColor = texture(texture1, TexCoord);
  if (texColor.a < 0.1)
    discard;
  frag_color = texColor;
}
