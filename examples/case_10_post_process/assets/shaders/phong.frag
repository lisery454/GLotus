// 输入
in vec3 frag_position; // 片段位置
in vec3 frag_normal;   // 片段法线

// 输出颜色
out vec4 frag_color;

struct Material {
  vec3 ambient_factor;      // 材质环境光反射系数
  vec3 diffuse_factor;      // 材质漫反射系数
  vec3 specular_factor;     // 材质镜面反射系数
  float specular_shininess; // 材质高光指数（反光度）
  vec3 diff_color;          // 漫反射颜色
  vec3 spec_color;          // 高光颜色
};

uniform Material material;

void main() {
  vec3 normal = normalize(frag_normal);
  vec3 view_dir = normalize(VIEW_POS - frag_position);

  PhongParams pp;
  pp.ambient_factor = material.ambient_factor;
  pp.diffuse_factor = material.diffuse_factor;
  pp.specular_factor = material.specular_factor;
  pp.specular_shininess = material.specular_shininess;
  pp.diff_color = material.diff_color;
  pp.spec_color = material.spec_color;

  vec4 result = CalcPhong(normal, view_dir, frag_position, pp);
  frag_color = result;
}