// 输入
in vec3 frag_position; // 片段位置
in vec3 frag_normal;   // 片段法线

// 输出颜色
out vec4 frag_color;

void main() { 
    frag_color = vec4(0.04, 0.28, 0.26, 1.0);
}