#version 140
uniform vec2 resolution;
uniform float scale;
uniform vec2 offset;
in vec2 position;
in vec3 color;
out vec4 fragColor;
void main() {
    vec4 tmp = vec4((position + offset)/resolution * scale, 0.0, 1.0);
    tmp.y = -tmp.y;
    gl_Position = tmp;
    fragColor = vec4(color, 1.0);
}