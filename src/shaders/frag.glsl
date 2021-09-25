#version 450

layout(location = 0) in vec3 fragColor;
layout(location = 1) in vec3 fragNormal;
layout(location = 2) in vec3 fragPosition;
layout(location = 3) in vec3 lightPosition;
layout(location = 4) in mat4 fragModel;

layout(location = 0) out vec4 o_Color;

void main() {
    mat3 normalMatrix = transpose(inverse(mat3(fragModel)));
    vec3 normal = normalize(normalMatrix * fragNormal);

    vec3 position = vec3(fragModel * vec4(fragPosition, 1));

    vec3 surfaceToLight = lightPosition - position; // hardcoded light @ (1.0, 1.0, 1.0)

    float brightness = dot(normal, surfaceToLight) / (length(surfaceToLight) * length(normal));
    brightness = clamp(brightness + 0.1, 0, 1); // add ambient

    vec4 surfaceColor = vec4(fragColor, 1.0);

	o_Color = vec4(brightness * 0.2 * surfaceColor.rgb, 1.0);
}
