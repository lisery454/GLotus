out vec2 TexCoords;

void main() {
  TexCoords = TEXCOORD;
  gl_Position =
      PROJECTION_MATRIX * VIEW_MATRIX * INSTANCE_MATRIX * vec4(POSITION, 1.0f);
}
