use glam::*;
use ndarray::*;
use crate::world::*;
use crate::Vert;
pub fn march(data: ArrayView2<Cell>, target: f32) -> Vec<Vert> {
    let (n, e, s, w, nw, ne, se, sw) = (0, 1, 2, 3, 4, 5, 6, 7);
    let lookup: [Vec<usize>; 16] = [
        vec![],                                         // 0
        vec![sw, w, s],                                 // 1
        vec![se, s, e],                                 // 2
        vec![w, e, se,  sw, se, w],                     // 3
        vec![n, ne, e],                                 // 4
        vec![n, ne, e,  sw, w, s,  w, n, e,  w, s, e],  // 5
        vec![n, ne, se,  n, s, se],                     // 6
        vec![w, sw, se,  w, n, se,  n, ne, se],         // 7
        vec![nw, w, n],                                 // 8
        vec![nw, n, s,  sw, nw, s],                     // 9
        vec![nw, w, n,  se, s, e,  w, n, e,  w, s, e],  // 10
        vec![nw, n, sw,  n, e, sw,  sw, e, se],         // 11
        vec![nw, ne, w,  w, e, ne],                     // 12
        vec![nw, ne, e,  nw, s, e,  nw, sw, s],         // 13
        vec![ne, nw, w,  ne, w, s,  ne, s, se],         // 14
        vec![nw, ne, sw,  ne, sw, se],                  // 15
    ];    
    let width = data.dim().0;
    let height = data.dim().1;
    let mut mesh: Vec<Vert> = Vec::new();        
    
    for y in 0..(height-1) {
        for x in 0..(width-1) {      
            let offset = Vec2::new(x as f32, y as f32);      
            let corners: [f32; 4] = [
                                        data.get((x,     y))    .unwrap().value, 
                                        data.get((x + 1, y))    .unwrap().value, 
                                        data.get((x + 1, y + 1)).unwrap().value,
                                        data.get((x,     y + 1)).unwrap().value
                                     ];  
            let case: usize =   (corners[0] > target) as usize * 8 +
                                (corners[1] > target) as usize * 4 +
                                (corners[2] > target) as usize * 2 +
                                (corners[3] > target) as usize;
            let verts: [Vec2; 8] = [
                Vec2::new((corners[0]-target)/(corners[0]-corners[1]), 0.0),
                Vec2::new(1.0, (corners[1]-target)/(corners[1]-corners[2])),
                Vec2::new((corners[3]-target)/(corners[3]-corners[2]), 1.0),
                Vec2::new(0.0, (corners[0]-target)/(corners[0]-corners[3])),
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(0.0, 1.0),
            ];            
            for id in lookup[case].iter() {
                let pos = verts[*id] + offset;
                mesh.push(Vert {
                    position: [pos.x, pos.y],
                    color: [1.0, 0.5, 0.0]
                });
            }
        }       
        
    }
    mesh
}


// Maybe finish one day (Probably not)
/*pub fn march(data: ArrayView2<f32>, target: f32) -> (Vec<Vec2>, Vec<u32>) {
    let (n, e, s, w, nw, ne, se, sw) = (0, 1, 2, 3, 4, 5, 6, 7);
    let lookup: [Vec<usize>; 16] = [
        vec![],                                         // 0
        vec![sw, w, s],                                 // 1
        vec![se, s, e],                                 // 2
        vec![w, e, se,  sw, se, w],                     // 3
        vec![n, ne, e],                                 // 4
        vec![n, ne, e,  sw, w, s,  w, n, e,  w, s, e],  // 5
        vec![n, ne, se,  n, s, se],                     // 6
        vec![w, sw, se,  w, n, se,  n, ne, se],         // 7
        vec![nw, w, n],                                 // 8
        vec![nw, n, s,  sw, nw, s],                     // 9
        vec![nw, w, n,  se, s, e,  w, n, e,  w, s, e],  // 10
        vec![nw, n, sw,  n, e, sw,  sw, e, se],         // 11
        vec![nw, ne, w,  w, e, ne],                     // 12
        vec![nw, ne, e,  nw, s, e,  nw, sw, s],         // 13
        vec![ne, nw, w,  ne, w, s,  ne, s, se],         // 14
        vec![nw, ne, sw,  ne, sw, se],                  // 15
    ];    
    let width = data.dim().0;
    let height = data.dim().1;
    let mut mesh: Vec<Vec2> = Vec::new();    
    let mut indices: Vec<u32> = Vec::new();
    let mut shared_indices: HashMap<(usize, usize), [Option<usize>; 8]> = HashMap::new();
    for y in 0..(height-1) {
        for x in 0..(width-1) {      
            let offset = Vec2::new(x as f32, y as f32);      
            let corners: [f32; 4] = [
                                        *data.get((x,     y))    .unwrap(), 
                                        *data.get((x + 1, y))    .unwrap(), 
                                        *data.get((x + 1, y + 1)).unwrap(),
                                        *data.get((x,     y + 1)).unwrap()
                                     ];  
            let case: usize =   (corners[0] > target) as usize * 8 +
                                (corners[1] > target) as usize * 4 +
                                (corners[2] > target) as usize * 2 +
                                (corners[3] > target) as usize;
            let verts: [Vec2; 8] = [
                Vec2::new((corners[0]-target)/(corners[0]-corners[1]), 0.0),
                Vec2::new(1.0, (corners[1]-target)/(corners[1]-corners[2])),
                Vec2::new((corners[3]-target)/(corners[3]-corners[2]), 1.0),
                Vec2::new(0.0, (corners[0]-target)/(corners[0]-corners[3])),
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 0.0),
                Vec2::new(1.0, 1.0),
                Vec2::new(0.0, 1.0),
            ];            
            let shared_ref = shared_indices.get(&(x, y));            
            let mut cell_indices: [Option<usize>; 8] = [None; 8];
            if let Some(i) = shared_ref {
                
                cell_indices = *i;
            }
            println!("indices: {:?}, x: {1}, y: {2}", cell_indices, x, y);
            if x == 0 {
                cell_indices[nw] = Some(mesh.len());
                mesh.push(verts[nw] + offset);
                cell_indices[w] = Some(mesh.len());
                mesh.push(verts[w] + offset);
                cell_indices[sw] = Some(mesh.len());
                mesh.push(verts[sw] + offset);
            } 
            if y == 0 {
                cell_indices[nw] = Some(mesh.len());
                mesh.push(verts[nw] + offset);
                cell_indices[n] = Some(mesh.len());
                mesh.push(verts[n] + offset);
                cell_indices[ne] = Some(mesh.len());
                mesh.push(verts[ne] + offset);
            }
            if lookup[case].contains(&s) {
                cell_indices[s] = Some(mesh.len());
                mesh.push(verts[s] + offset);
            }
            if lookup[case].contains(&se) {
                cell_indices[se] = Some(mesh.len());
                mesh.push(verts[s] + offset);
            }
            if lookup[case].contains(&e) {
                cell_indices[e] = Some(mesh.len());
                mesh.push(verts[s] + offset);
            }            
            let mut share_right: [Option<usize>; 8] = [None; 8];
            let mut share_down: [Option<usize>; 8] = [None; 8];
            
            share_right[nw] = cell_indices[ne];
            share_right[w] = cell_indices[e];
            share_right[sw] = cell_indices[se];
            
            share_down[nw] = cell_indices[sw];
            share_down[n] = cell_indices[s];
            share_down[ne] = cell_indices[se];

            shared_indices.insert((x + 1, y), share_right);
            shared_indices.insert((x, y + 1), share_down);
            // println!("indices: {:?}, x: {1}, y: {2}", cell_indices, x, y);
            for id in lookup[case].iter() {
                indices.push(cell_indices[*id].unwrap() as u32);
            }
        }       
        
    }
    (mesh, indices)    
}*/