#version 140
uniform vec2 resolution;
uniform float scale;
uniform vec2 offset;
in vec2 position;
in vec3 color;
out vec4 fragColor;
void main() {
    gl_Position = vec4((position + offset)/resolution * scale, 0.0, 1.0);
    fragColor = vec4(color, 1.0);
}