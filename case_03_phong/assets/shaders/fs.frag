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

void main() {
  vec3 normal = normalize(normal_in_world);
  vec3 view_dir = normalize(g_view_position - frag_pos_in_world);

  vec3 diff_tex = texture(material.diffuse_texture, tex_coord).rgb;
  vec3 spec_tex = texture(material.specular_texture, tex_coord).rgb;

  PhongParams pp;
  pp.ambient_factor = material.ambient_factor;
  pp.diffuse_factor = material.diffuse_factor;
  pp.specular_factor = material.specular_factor;
  pp.specular_shininess = material.specular_shininess;
  pp.diff_color = diff_tex;
  pp.spec_color = spec_tex;

  vec3 result = CalcPhong(normal, view_dir, frag_pos_in_world, pp);
  frag_color = vec4(result, 1.0);
}