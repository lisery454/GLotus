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
struct PhongParams {
  vec3 ambient_factor;      // 材质环境光反射系数
  vec3 diffuse_factor;      // 材质漫反射系数
  vec3 specular_factor;     // 材质镜面反射系数
  float specular_shininess; // 材质高光指数（反光度）
  vec3 diff_color;          // 漫反射固有色
  vec3 spec_color;          // 高光固有色
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

vec3 CalcDirectionalLight(Light L, vec3 normal, vec3 view_dir, PhongParams pp) {
  vec3 light_dir = normalize(-L.direction.xyz);
  float diff = max(dot(normal, light_dir), 0.0);

  vec3 reflect_dir = reflect(-light_dir, normal);
  float spec = pow(max(dot(view_dir, reflect_dir), 0.0), pp.specular_shininess);

  // 只返回漫反射和镜面反射
  return pp.diffuse_factor * diff * pp.diff_color * L.intensity +
         pp.specular_factor * spec * pp.spec_color * L.intensity;
}

vec3 CalcPointLight(Light L, vec3 normal, vec3 view_dir, vec3 frag_pos,
                    PhongParams pp) {
  vec3 light_dir = normalize(L.position.xyz - frag_pos);
  float diff = max(dot(normal, light_dir), 0.0);

  vec3 reflect_dir = reflect(-light_dir, normal);
  float spec = pow(max(dot(view_dir, reflect_dir), 0.0), pp.specular_shininess);

  float distance = length(L.position.xyz - frag_pos);
  float constant = 1.0;
  float linear = 0.09;
  float quadratic = 0.032;
  float attenuation =
      1.0 / (constant + linear * distance + quadratic * distance * distance);

  return attenuation *
         (pp.diffuse_factor * diff * pp.diff_color * L.intensity +
          pp.specular_factor * spec * pp.spec_color * L.intensity);
}

vec3 CalcSpotLight(Light L, vec3 normal, vec3 view_dir, vec3 frag_pos,
                   PhongParams pp) {
  vec3 light_dir = normalize(L.position.xyz - frag_pos);
  float diff = max(dot(normal, light_dir), 0.0);

  float theta = dot(light_dir, normalize(-L.direction.xyz));
  float epsilon = L.inner_cone - L.outer_cone;
  float spot_factor = clamp((theta - L.outer_cone) / epsilon, 0.0, 1.0);

  vec3 reflect_dir = reflect(-light_dir, normal);
  float spec = pow(max(dot(view_dir, reflect_dir), 0.0), pp.specular_shininess);

  float distance = length(L.position.xyz - frag_pos);
  float attenuation = clamp(1.0 - distance / L.range, 0.0, 1.0);

  return attenuation * spot_factor *
         (pp.diffuse_factor * diff * pp.diff_color * L.intensity +
          pp.specular_factor * spec * pp.spec_color * L.intensity);
}

vec4 CalcPhong(vec3 normal, vec3 view_dir, vec3 frag_pos, PhongParams pp) {
  // 环境光只计算一次
  vec3 ambient = pp.ambient_factor * pp.diff_color;
  vec3 result = ambient;

  for (int i = 0; i < LIGHT_COUNT; ++i) {
    Light L = LIGHTS[i];
    if (L.light_type == 0)
      result += L.color.rgb * CalcDirectionalLight(L, normal, view_dir, pp);
    else if (L.light_type == 1)
      result += L.color.rgb * CalcPointLight(L, normal, view_dir, frag_pos, pp);
    else if (L.light_type == 2)
      result += L.color.rgb * CalcSpotLight(L, normal, view_dir, frag_pos, pp);
  }

  return vec4(result, 1.0);
}

float LinearizeDepth(float glFragCoordZ) {
  float near = CAMERA.near_plane;
  float far = CAMERA.far_plane;
  float z = glFragCoordZ * 2.0 - 1.0;
  return (2.0 * near * far) / (far + near - z * (far - near)) / far;
}