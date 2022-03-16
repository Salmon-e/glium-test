#version 140
uniform vec2 resolution;
uniform float scale;
uniform vec2 offset;
in vec2 position;
in vec4 corners;
out vec2 fragPos;
out vec4 fragCorners;
out vec4 fragColor;
void main() {
    vec4 tmp = vec4((position + offset)/resolution * scale, 0.0, 1.0);
    tmp.y = -tmp.y;
    gl_Position = tmp;    
    fragPos = position;
    fragCorners = corners;
}