out vec4 FragColor;

in vec3 frag_texcoords;

uniform samplerCube skybox;

void main() { FragColor = texture(skybox, frag_texcoords); }