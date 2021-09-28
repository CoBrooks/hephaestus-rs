#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;
layout(location = 2) in vec3 normal;
layout(location = 3) in vec2 uv;

layout(location = 0) out vec3 out_color;
layout(location = 1) out vec3 out_normal;
layout(location = 2) out vec2 texCoord;

layout(set = 0, binding = 0) uniform VPData {
    mat4 view;
    mat4 proj;
} vp;

layout(set = 1, binding = 0) uniform ModelData {
    mat4 model;
    mat4 normals;
} model;

void main() {
    gl_Position = vp.proj * vp.view * model.model * vec4(position, 1.0);
    out_color = color;
    out_normal = mat3(model.normals) * normal;
    texCoord = uv * vec2(1.0, -1.0);
}
