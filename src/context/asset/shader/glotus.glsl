#version 460 core
uniform mat3 g_normal_matrix;     // 法线从模型空间到世界空间的矩阵
uniform mat4 g_model_matrix;      // 点从模型空间到世界空间的矩阵
uniform mat4 g_view_matrix;       // 点从世界空间到摄像机空间的矩阵
uniform mat4 g_projection_matrix; // 点从摄像机空间到透视空间的矩阵

uniform vec3 g_view_position; // 视角（摄像机）的位置

struct Light {
  int light_type;
  vec4 color;
  vec3 position;
  vec3 direction;
  float intensity;
  float range;
  float inner_cone;
  float outer_cone;
};

uniform int g_light_count;
uniform Light g_lights[16];

struct PhongParams {
  vec3 ambient_factor;      // 材质环境光反射系数
  vec3 diffuse_factor;      // 材质漫反射系数
  vec3 specular_factor;     // 材质镜面反射系数
  float specular_shininess; // 材质高光指数（反光度）
  vec3 diff_color;          // 漫反射固有色
  vec3 spec_color;          // 高光固有色
};

vec3 CalcDirectionalLight(Light L, vec3 normal, vec3 view_dir, PhongParams pp) {
  vec3 light_dir = normalize(-L.direction);

  float diff = max(dot(normal, light_dir), 0.0);

  vec3 reflect_dir = reflect(-light_dir, normal);
  float spec = pow(max(dot(view_dir, reflect_dir), 0.0), pp.specular_shininess);

  return pp.ambient_factor * pp.diff_color +
         pp.diffuse_factor * diff * pp.diff_color * L.intensity +
         pp.specular_factor * spec * pp.spec_color * L.intensity;
}

vec3 CalcPointLight(Light L, vec3 normal, vec3 view_dir, vec3 frag_pos,
                    PhongParams pp) {
  vec3 light_dir = normalize(L.position - frag_pos);

  float diff = max(dot(normal, light_dir), 0.0);

  vec3 reflect_dir = reflect(-light_dir, normal);
  float spec = pow(max(dot(view_dir, reflect_dir), 0.0), pp.specular_shininess);

  // 距离衰减
  float distance = length(L.position - frag_pos);
  float constant = 1.0;
  float linear = 0.09;     // 可调
  float quadratic = 0.032; // 可调
  float attenuation =
      1.0 / (constant + linear * distance +
             quadratic * distance * distance); // 标准点光衰弱公式
  //   float attenuation = clamp(1.0 - distance / L.range, 0.0, 1.0);  线性衰减

  return pp.ambient_factor * pp.diff_color +
         attenuation *
             (pp.diffuse_factor * diff * pp.diff_color * L.intensity +
              pp.specular_factor * spec * pp.spec_color * L.intensity);
}

vec3 CalcSpotLight(Light L, vec3 normal, vec3 view_dir, vec3 frag_pos,
                   PhongParams pp) {
  vec3 light_dir = normalize(L.position - frag_pos);

  float diff = max(dot(normal, light_dir), 0.0);

  float theta = dot(light_dir, normalize(-L.direction));
  float epsilon = L.inner_cone - L.outer_cone;
  float spot_factor = clamp((theta - L.outer_cone) / epsilon, 0.0, 1.0);

  vec3 reflect_dir = reflect(-light_dir, normal);
  float spec = pow(max(dot(view_dir, reflect_dir), 0.0), pp.specular_shininess);

  float distance = length(L.position - frag_pos);
  float attenuation = clamp(1.0 - distance / L.range, 0.0, 1.0);

  return pp.ambient_factor * pp.diff_color +
         attenuation * spot_factor *
             (pp.diffuse_factor * diff * pp.diff_color * L.intensity +
              pp.specular_factor * spec * pp.spec_color * L.intensity);
}

// normal 世界坐标中的法线
// view_dir 世界坐标中视角方向
// frag_pos 世界坐标中的frag位置
// pp 参数
vec4 CalcPhong(vec3 normal, vec3 view_dir, vec3 frag_pos, PhongParams pp) {
  vec4 result = vec4(0);

  for (int i = 0; i < g_light_count; ++i) {
    Light L = g_lights[i];
    if (L.light_type == 0)
      result +=
          L.color * vec4(CalcDirectionalLight(L, normal, view_dir, pp), 1.0);
    else if (L.light_type == 1)
      result += L.color *
                vec4(CalcPointLight(L, normal, view_dir, frag_pos, pp), 1.0);
    else if (L.light_type == 2)
      result +=
          L.color * vec4(CalcSpotLight(L, normal, view_dir, frag_pos, pp), 1.0);
  }

  return result;
}

struct Camera {
  int camera_type;
  vec3 direction;
  vec3 position;
  float aspect_ratio;
  float near_plane;
  float far_plane;
};

uniform Camera g_camera;

float LinearizeDepth(float glFragCoordZ) {
  float near = g_camera.near_plane;
  float far = g_camera.far_plane;
  float z = glFragCoordZ * 2.0 - 1.0;
  return (2.0 * near * far) / (far + near - z * (far - near)) / far;
}