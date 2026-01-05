out vec4 FragColor;

in vec3 frag_normal;
in vec3 frag_position;

uniform samplerCube skybox;

void main() {
  vec3 I = normalize(frag_position - VIEW_POS);
  vec3 R = reflect(I, normalize(frag_normal));
  FragColor = vec4(texture(skybox, R).rgb, 1.0);
}