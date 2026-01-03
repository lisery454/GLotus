out vec2 TexCoord;

void main() {
  gl_Position = vec4(POSITION, 1.0f);
  TexCoord = TEXCOORD;
}