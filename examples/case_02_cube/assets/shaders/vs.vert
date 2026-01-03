out vec2 TexCoord;

void main() {
  gl_Position = PVM_MATRIX * vec4(POSITION, 1.0f);
  TexCoord = TEXCOORD;
}