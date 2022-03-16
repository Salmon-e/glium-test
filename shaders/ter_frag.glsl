#version 140

in vec2 fragPos;
in vec4 fragCorners;
out vec4 color;
uniform sampler3D tex;
void main() {
    vec2 subPos = fragPos - floor(fragPos);
    vec2 vz = vec2(0, 0);
    vec2 vw = vec2(1, 0);
    vec2 vx = vec2(1, 1);
    vec2 vy = vec2(0, 1);
    float wx = distance(subPos, vx);
    float wy = distance(subPos, vy);
    float wz = distance(subPos, vz);
    float ww = distance(subPos, vw);
    float sum = wx + wy + wz + ww;
    wx /= sum;
    wy /= sum;
    wz /= sum;
    ww /= sum;

    vec4 cx = texture(tex, vec3(fragPos / 5, fragCorners.x));
    vec4 cy = texture(tex, vec3(fragPos / 5, fragCorners.y));
    vec4 cz = texture(tex, vec3(fragPos / 5, fragCorners.z));
    vec4 cw = texture(tex, vec3(fragPos / 5, fragCorners.w));
    color = cx * wx + cy * wy + cz * wz + cw * ww;
}