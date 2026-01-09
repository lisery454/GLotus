layout(triangles) in;
layout(line_strip, max_vertices = 6) out;

in VS_OUT { vec3 normal; }
gs_in[];

const float MAGNITUDE = 0.2;

void GenerateLine(int index) {
  // 此时 gl_in[index].gl_Position 是世界坐标
  vec4 worldPos = gl_in[index].gl_Position;

  // 起点
  gl_Position = PROJECTION_MATRIX * VIEW_MATRIX * worldPos;
  EmitVertex();

  // 终点：在世界空间进行位移，再投影
  vec4 worldNormalEnd = worldPos + vec4(gs_in[index].normal * MAGNITUDE, 0.0);
  gl_Position = PROJECTION_MATRIX * VIEW_MATRIX * worldNormalEnd;
  EmitVertex();

  EndPrimitive();
}

void main() {
  GenerateLine(0);
  GenerateLine(1);
  GenerateLine(2);
}