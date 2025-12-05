out vec4 frag_color;

in vec3 normal_in_world;
in vec3 frag_pos_in_world;
in vec2 tex_coord;

struct Material {
  vec3 ambient_factor;        // 材质环境光反射系数
  vec3 diffuse_factor;        // 材质漫反射系数
  vec3 specular_factor;       // 材质镜面反射系数
  float specular_shininess;   // 材质高光指数（反光度）
  sampler2D diffuse_texture;  // 漫反射贴图
  sampler2D specular_texture; // 高光贴图
};

uniform Material material;

vec3 CalcDirectionalLight(Light L, vec3 normal, vec3 view_dir, vec3 diff_tex,
                          vec3 spec_tex) {
  vec3 light_dir = normalize(-L.direction);

  float diff = max(dot(normal, light_dir), 0.0);

  vec3 reflect_dir = reflect(-light_dir, normal);
  float spec =
      pow(max(dot(view_dir, reflect_dir), 0.0), material.specular_shininess);

  return material.ambient_factor * diff_tex +
         material.diffuse_factor * diff * diff_tex * L.intensity +
         material.specular_factor * spec * spec_tex * L.intensity;
}

vec3 CalcPointLight(Light L, vec3 normal, vec3 view_dir, vec3 diff_tex,
                    vec3 spec_tex) {
  vec3 light_dir = normalize(L.position - frag_pos_in_world);

  float diff = max(dot(normal, light_dir), 0.0);

  vec3 reflect_dir = reflect(-light_dir, normal);
  float spec =
      pow(max(dot(view_dir, reflect_dir), 0.0), material.specular_shininess);

  // 距离衰减
  float distance = length(L.position - frag_pos_in_world);
  float attenuation = clamp(1.0 - distance / L.range, 0.0, 1.0);

  return attenuation *
         (material.ambient_factor * diff_tex +
          material.diffuse_factor * diff * diff_tex * L.intensity +
          material.specular_factor * spec * spec_tex * L.intensity);
}

vec3 CalcSpotLight(Light L, vec3 normal, vec3 view_dir, vec3 diff_tex,
                   vec3 spec_tex) {
  vec3 light_dir = normalize(L.position - frag_pos_in_world);

  float diff = max(dot(normal, light_dir), 0.0);

  float theta = dot(light_dir, normalize(-L.direction));
  float epsilon = L.inner_cone - L.outer_cone;
  float spot_factor = clamp((theta - L.outer_cone) / epsilon, 0.0, 1.0);

  vec3 reflect_dir = reflect(-light_dir, normal);
  float spec =
      pow(max(dot(view_dir, reflect_dir), 0.0), material.specular_shininess);

  float distance = length(L.position - frag_pos_in_world);
  float attenuation = clamp(1.0 - distance / L.range, 0.0, 1.0);

  return attenuation * spot_factor *
         (material.ambient_factor * diff_tex +
          material.diffuse_factor * diff * diff_tex * L.intensity +
          material.specular_factor * spec * spec_tex * L.intensity);
}

vec3 CalcPhong(vec3 normal, vec3 view_dir, vec3 diff_tex, vec3 spec_tex) {
  vec3 result = vec3(0);

  for (int i = 0; i < g_light_count; ++i) {
    Light L = g_lights[i];
    if (L.light_type == 0)
      result += L.color *
                CalcDirectionalLight(L, normal, view_dir, diff_tex, spec_tex);
    else if (L.light_type == 1)
      result +=
          L.color * CalcPointLight(L, normal, view_dir, diff_tex, spec_tex);
    else if (L.light_type == 2)
      result +=
          L.color * CalcSpotLight(L, normal, view_dir, diff_tex, spec_tex);
  }

  return result;
}

void main() {
  vec3 normal = normalize(normal_in_world);
  vec3 view_dir = normalize(g_view_position - frag_pos_in_world);

  vec3 diff_tex = texture(material.diffuse_texture, tex_coord).rgb;
  vec3 spec_tex = texture(material.specular_texture, tex_coord).rgb;

  vec3 result = CalcPhong(normal, view_dir, diff_tex, spec_tex);

  frag_color = vec4(result, 1.0);
}