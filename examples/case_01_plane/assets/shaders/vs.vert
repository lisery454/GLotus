layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 texcoord;

void main() {
  gl_Position =
      g_projection_matrix * g_view_matrix * g_model_matrix * vec4(position, 1.);
}