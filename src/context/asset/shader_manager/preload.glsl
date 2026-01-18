#version 460 core

// struct defination
struct Camera {
  vec4 position; // 0

  vec4 direction; // 16

  int camera_type;    // 32
  float fov;          // 36
  float aspect_ratio; // 40
  float near_plane;   // 44

  float far_plane; // 48
  int _pad0;
  int _pad1;
  int _pad2; // 52 (显式填充，确保总长 64B)
};

struct Light {
  // 16B
  int light_type;
  //   int _pad[3];
  int _pad0;
  int _pad1;
  int _pad2;
  // 16B
  vec4 color;
  // 16B
  vec4 position;
  // 16B
  vec4 direction;
  // 16B
  float intensity;
  float range;
  float inner_cone;
  float outer_cone;
};

// UBO
// Binding 0: 每一帧更新一次
layout(std140, binding = 0) uniform FrameData {
  int light_count;
  int _pad0;
  int _pad1;
  int _pad2;
  Light lights[16];
}
g_frame;

// Binding 1: 每个相机更新一次
layout(std140, binding = 1) uniform CameraData {
  mat4 view_matrix;       // 点从世界空间到摄像机空间的矩阵
  mat4 projection_matrix; // 点从摄像机空间到透视空间的矩阵
  Camera camera;
}
g_camera;

// Binding 2: 每个模型更新一次
layout(std140, binding = 2) uniform ModelData {
  mat4 model_matrix;  // 点从模型空间到世界空间的矩阵
  mat4 normal_matrix; // 法线从模型空间到世界空间的矩阵
}
g_model;

#define MODEL_MATRIX g_model.model_matrix
#define NORMAL_MATRIX mat3(g_model.normal_matrix)
#define PROJECTION_MATRIX g_camera.projection_matrix
#define VIEW_MATRIX g_camera.view_matrix
#define PVM_MATRIX PROJECTION_MATRIX *VIEW_MATRIX *MODEL_MATRIX
#define CAMERA g_camera.camera
#define VIEW_POS CAMERA.position.xyz
#define LIGHT_COUNT g_frame.light_count
#define LIGHTS g_frame.lights

