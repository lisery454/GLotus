out VS_OUT { vec3 normal; }
vs_out;

void main() {
  vec4 worldPos = MODEL_MATRIX * vec4(POSITION, 1.0);
  gl_Position = worldPos;
  vs_out.normal = normalize(NORMAL_MATRIX * NORMAL);
}