layout(location = 0) in vec3 position;   // 位置（必需）
layout(location = 1) in vec3 normal;     // 法线（如果有）
layout(location = 2) in vec3 tangent;    // 切线（如果有）
layout(location = 3) in vec3 bitangent;  // 副切线（如果有）
layout(location = 4) in vec2 texcoord;   // 2D UV坐标（如果有）- 注意是vec2
layout(location = 5) in vec3 texcoord3d; // 3D UV坐标（如果有）
layout(location = 6) in vec3 color;      // 顶点颜色（如果有）

#define POSITION position
#define NORMAL normal
#define TANGENT tangent
#define BITANGENT bitangent
#define TEXCOORD texcoord
#define TEXCOORD3D texcoord3d
#define COLOR color