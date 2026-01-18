out vec4 frag_color;

in vec3 normal_in_world;
in vec3 frag_pos_in_world;
in vec2 tex_coord;

struct PhongParams {
  vec3 ambient_factor;      // 材质环境光反射系数
  vec3 diffuse_factor;      // 材质漫反射系数
  vec3 specular_factor;     // 材质镜面反射系数
  float specular_shininess; // 材质高光指数（反光度）
  vec3 diff_color;          // 漫反射固有色
  vec3 spec_color;          // 高光固有色
};

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

struct Material {
  vec3 ambient_factor;        // 材质环境光反射系数
  vec3 diffuse_factor;        // 材质漫反射系数
  vec3 specular_factor;       // 材质镜面反射系数
  float specular_shininess;   // 材质高光指数（反光度）
  sampler2D diffuse_texture;  // 漫反射贴图
  sampler2D specular_texture; // 高光贴图
};

uniform Material material;

void main() {
  vec3 normal = normalize(normal_in_world);
  vec3 view_dir = normalize(VIEW_POS - frag_pos_in_world);

  vec3 diff_tex = texture(material.diffuse_texture, tex_coord).rgb;
  vec3 spec_tex = texture(material.specular_texture, tex_coord).rgb;

  PhongParams pp;
  pp.ambient_factor = material.ambient_factor;
  pp.diffuse_factor = material.diffuse_factor;
  pp.specular_factor = material.specular_factor;
  pp.specular_shininess = material.specular_shininess;
  pp.diff_color = diff_tex;
  pp.spec_color = spec_tex;

  vec4 result = CalcPhong(normal, view_dir, frag_pos_in_world, pp);
  frag_color = result;
}