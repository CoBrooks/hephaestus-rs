#version 450

layout(set = 1, binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 color;
layout(location = 2) in vec3 normal;
layout(location = 3) in vec2 uv;

layout(location = 0) out vec3 fragColor;
layout(location = 1) out vec3 fragNormal;
layout(location = 2) out vec3 fragPosition;
layout(location = 3) out vec3 lightPosition;
layout(location = 4) out vec2 texCoords;
layout(location = 5) out mat4 fragModel;

void main() {
	gl_Position = ubo.proj * ubo.view * ubo.model * vec4(position, 1.0);

    fragColor = color;
    fragNormal = normal;
    fragModel = ubo.model;
    fragPosition = position;
    lightPosition = vec3(ubo.model * vec4(0.0, 1.5, 1.0, 1.0));
    texCoords = uv;
}
