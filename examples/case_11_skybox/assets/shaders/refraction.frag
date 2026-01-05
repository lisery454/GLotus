out vec4 FragColor;

in vec3 frag_normal;
in vec3 frag_position;

uniform samplerCube skybox;

void main() {
  float ratio = 1.00 / 1.52;
  vec3 I = normalize(frag_position - VIEW_POS);
  vec3 R = refract(I, normalize(frag_normal), ratio);
  FragColor = vec4(texture(skybox, R).rgb, 1.0);
}