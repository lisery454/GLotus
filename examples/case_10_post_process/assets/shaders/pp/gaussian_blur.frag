in vec2 TexCoord;
out vec4 FragColor;

uniform sampler2D screenTexture;

// 偏移量，可以根据需要调整。通常 resize 后这里传入 1.0 / resolution
const float offset = 0.002;

void main() {
  // 定义 3x3 邻域的采样偏移
  vec2 offsets[9] =
      vec2[](vec2(-offset, offset), vec2(0.0, offset), vec2(offset, offset),
             vec2(-offset, 0.0), vec2(0.0, 0.0), vec2(offset, 0.0),
             vec2(-offset, -offset), vec2(0.0, -offset), vec2(offset, -offset));

  // 定义高斯核权重 (3x3 Kernel)
  // 中间权重最高(4/16)，四周次之(2/16)，四个角最低(1/16)
  float kernel[9] =
      float[](1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0, 2.0 / 16.0, 4.0 / 16.0,
              2.0 / 16.0, 1.0 / 16.0, 2.0 / 16.0, 1.0 / 16.0);

  vec3 color = vec3(0.0);
  for (int i = 0; i < 9; i++) {
    color += texture(screenTexture, TexCoord + offsets[i]).rgb * kernel[i];
  }

  FragColor = vec4(color, 1.0);
}