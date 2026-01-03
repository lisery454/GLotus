out vec3 frag_texcoords; // 片段UV

void main() {
  frag_texcoords = POSITION;
  vec4 pos = PROJECTION_MATRIX * mat4(mat3(VIEW_MATRIX)) * vec4(POSITION, 1.0);
  gl_Position = pos.xyww;
}