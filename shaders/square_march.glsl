#version 430
layout(local_size_x=32, local_size_y=32, local_size_z=1) in;
shared uint index;
struct Cell {
    float value;
    float texture;
};
uniform Cells {
    Cell cells[1024];
};
struct Vert {
    vec2 position;
    vec4 corners;
};
buffer outb {
    Vert verts[12288];
};
void main() {
    // Random junk code
    vec2 id = gl_LocalInvocationID.xy;
    uint cell_i = uint(id.y * 32 + id.x);
    float cell_val = cells[cell_i].value;a3r
    verts[cell_i] = Vert(vec2(cell_val + 1.0, 0.7), vec4(0.0, 0.0, 0.0, 0.0));
}