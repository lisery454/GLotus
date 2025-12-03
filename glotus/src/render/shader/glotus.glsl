uniform mat3 normal_matrix; // 法线从模型空间到世界空间的矩阵
uniform mat4 model_matrix; // 点从模型空间到世界空间的矩阵
uniform mat4 view_matrix;   // 点从世界空间到摄像机空间的矩阵
uniform mat4 projection_matrix; // 点从摄像机空间到透视空间的矩阵

uniform vec3 view_position; // 视角（摄像机）的位置

struct Light {
    int  light_type;
    vec3 color;
    vec3 position;
    vec3 direction;
    float intensity;
    float range;
    float inner_cone;
    float outer_cone;
};

uniform int light_count;
uniform Light lights[16]; 
