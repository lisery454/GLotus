in vec2 TexCoord;
out vec4 FragColor;

uniform sampler2D screenTexture;

const float offset = 0.005;

void main() {
  // 分别对三个通道进行不同偏移的采样
  float r = texture(screenTexture, TexCoord + vec2(offset, 0.0)).r;
  float g = texture(screenTexture, TexCoord).g;
  float b = texture(screenTexture, TexCoord - vec2(offset, 0.0)).b;

  // 组合成最终颜色，Alpha 强制设为 1.0 避免透明度导致全黑
  FragColor = vec4(r, g, b, 1.0);
}