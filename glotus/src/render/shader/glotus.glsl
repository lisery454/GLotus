#version 460 core
uniform mat3 g_normal_matrix; // 法线从模型空间到世界空间的矩阵
uniform mat4 g_model_matrix;  // 点从模型空间到世界空间的矩阵
uniform mat4 g_view_matrix;   // 点从世界空间到摄像机空间的矩阵
uniform mat4 g_projection_matrix; // 点从摄像机空间到透视空间的矩阵

uniform vec3 g_view_position; // 视角（摄像机）的位置

struct Light {
  int light_type;
  vec3 color;
  vec3 position;
  vec3 direction;
  float intensity;
  float range;
  float inner_cone;
  float outer_cone;
};

uniform int g_light_count;
uniform Light g_lights[16];
