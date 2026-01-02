in vec2 TexCoord; // 范围通常是 [0, 1]
out vec4 FragColor;

uniform sampler2D screenTexture;

// 均匀强度，值越大，晕影越明显
uniform float vignetteStrength = 0.2;
// 晕影的范围，值越大，黑色区域越向中心收缩
uniform float vignetteRadius = 0.65;
// 晕影的平滑度，值越大，从亮到暗的过渡越锐利
uniform float vignetteSmoothness = 0.7;

void main() {
  // 1. 采样原始纹理颜色
  vec4 color = texture(screenTexture, TexCoord);

  // 2. 将纹理坐标转换到中心为 (0,0)，范围为 [-0.5, 0.5]
  // 这样方便计算距离中心点的距离
  vec2 uv_centered = TexCoord - 0.5;

  // 3. 计算距离中心点的欧几里得距离（或更简单的平方距离）
  // 为了让晕影更圆，通常使用 vec2(1.0, 1.0) 来拉伸uv_centered，避免椭圆形
  // 例如：vec2(1.0 / aspectRatio, 1.0)
  // 这里我们先假设正方形纹理或不考虑长宽比，直接计算
  float dist = length(
      uv_centered * 2.0); // 距离中心点，最大值约 sqrt(0.5*0.5*2) * 2 = sqrt(2)

  // 4. 根据距离计算晕影强度
  // remap(dist, min_dist, max_dist, 0.0, 1.0)
  // 距离越远，vignetteFactor 越大（接近 1.0）
  float vignetteFactor =
      smoothstep(vignetteRadius, vignetteRadius + vignetteSmoothness, dist);

  // 5. 将晕影强度应用到颜色上
  // (1.0 - vignetteFactor * vignetteStrength) 会让边缘区域的乘数变小，颜色变暗
  color.rgb *= (1.0 - vignetteFactor * vignetteStrength);

  FragColor = color;
}