out vec4 FragColor;

in vec2 TexCoord;

uniform sampler2D texture1;

void main() {
  vec4 c = texture(texture1, TexCoord);
  if (c.a < 0.1)
    discard;
  FragColor = c;
}