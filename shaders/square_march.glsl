#version 430
layout(local_size_x=32, local_size_y=32, local_size_z=1) in;
shared uint globalIndex;
struct Cell {
    float value;
    float texture;
};
layout(binding=0) uniform Cells {
    Cell cells[33*33];
};
struct Vert {
    vec2 position;
    vec4 corners;
};
layout(binding=1) buffer outb {
    Vert verts[12288];
};
float target = 0.0;
int n = 0;
int e = 1;
int s = 2;
int w = 3;
int nw = 4;
int ne = 5;
int se = 6;
int sw = 7;
int[16][12] lookup = {
        {-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1},   // 0
        {sw, w, s, -1, -1, -1, -1, -1, -1, -1, -1, -1},     // 1
        {se, s, e, -1, -1, -1, -1, -1, -1, -1, -1, -1},     // 2
        {w, e, se,  sw, se, w, -1, -1, -1, -1, -1, -1},     // 3
        {n, ne, e, -1, -1, -1, -1, -1, -1, -1, -1, -1},     // 4
        {n, ne, e,  sw, w, s,  w, n, e,  w, s, e},          // 5
        {n, ne, se,  n, s, se, -1, -1, -1, -1, -1, -1},     // 6
        {w, sw, se,  w, n, se,  n, ne, se, -1, -1, -1},     // 7
        {nw, w, n, -1, -1, -1, -1, -1, -1, -1, -1, -1},     // 8
        {nw, n, s,  sw, nw, s,  -1, -1, -1, -1, -1, -1},    // 9
        {nw, w, n,  se, s, e,  w, n, e,  w, s, e},          // 10
        {nw, n, sw,  n, e, sw,  sw, e, se, -1, -1, -1},     // 11
        {nw, ne, w,  w, e, ne, -1, -1, -1, -1, -1, -1},     // 12
        {nw, ne, e,  nw, s, e,  nw, sw, s, -1, -1, -1},     // 13
        {ne, nw, w,  ne, w, s,  ne, s, se, -1, -1, -1},     // 14
        {nw, ne, sw,  ne, sw, se, -1, -1, -1, -1, -1, -1}   // 15
}; 
uint index(vec2 pos) {
    return uint(pos.y * 33 + pos.x);
}
void main() {
    vec2 id = gl_LocalInvocationID.xy;
    if (id.x == 0 && id.y == 0) {
        globalIndex = 0;
    }
    barrier();
    // Store the corners
    Cell cell_corners[4] = {
        cells[index(id)],
        cells[index(id + vec2(1, 0))],
        cells[index(id + vec2(1, 1))],
        cells[index(id + vec2(0, 1))]
    };
    // Store the corner values
    float corners[4] = {
        cell_corners[0].value,
        cell_corners[1].value,
        cell_corners[2].value,
        cell_corners[3].value
    };
    vec4 texture = vec4(
        cell_corners[0].texture,
        cell_corners[1].texture,
        cell_corners[2].texture,
        cell_corners[3].texture
    );
    // calculate the case value
    uint c = uint((corners[0] > target)) * 8 +
             uint((corners[1] > target)) * 4 +
             uint((corners[2] > target)) * 2 +
             uint((corners[3] > target));
    // the 8 vertices that will be used to make the triangles of the cell
    vec2 v[8] = {                
                vec2((corners[0]-target)/(corners[0]-corners[1]), 0.0),
                vec2(1.0, (corners[1]-target)/(corners[1]-corners[2])),
                vec2((corners[3]-target)/(corners[3]-corners[2]), 1.0),
                vec2(0.0, (corners[0]-target)/(corners[0]-corners[3])),
                vec2(0.0, 0.0),
                vec2(1.0, 0.0),
                vec2(1.0, 1.0),
                vec2(0.0, 1.0)
    };    
    uint vcount;
    for(vcount = 0; vcount < 12 && lookup[c][vcount] != -1; vcount++);     
    uint insert = atomicAdd(globalIndex, vcount);
    for(int i = 0; i < vcount; i += 3) {        
        verts[insert + i]     = Vert(v[lookup[c][i]]     + id, texture);
        verts[insert + 1 + i] = Vert(v[lookup[c][i + 1]] + id, texture);
        verts[insert + 2 + i] = Vert(v[lookup[c][i + 2]] + id, texture);
    }
}
