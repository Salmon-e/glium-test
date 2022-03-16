use std::collections::HashMap;
use crate::world_mesh::{gvert, GVert};
use glam::*;
use ndarray::*;
use crate::world::*;

pub fn get_lines(data: [f32; 4], offset: Vec2, target: f32) -> Vec<Vec2> {

    let (n, e, s, w) = (0, 1, 2, 3);
    let lookup: [Vec<usize>; 16] = [
        vec![],             // 0
        vec![w, s],         // 1
        vec![s, e],         // 2
        vec![w, e],         // 3
        vec![n, e],         // 4
        vec![w, n,  s, e],  // 5
        vec![n, s],         // 6
        vec![w, n],         // 7
        vec![w, n],         // 8
        vec![n, s],         // 9
        vec![w, s,  n, e],  // 10
        vec![n, e],         // 11
        vec![w, e],         // 12
        vec![s, e],         // 13
        vec![w, s],         // 14
        vec![w, n,  n, e,  e, s,  s, w]  // 15
    ]; 
                              
    let case: usize =   (data[0] > target) as usize * 8 +
                        (data[1] > target) as usize * 4 +
                        (data[2] > target) as usize * 2 +
                        (data[3] > target) as usize;
    
    let verts: [Vec2; 4] = [
        Vec2::new((data[0]-target)/(data[0]-data[1]), 0.0),
        Vec2::new(1.0, (data[1]-target)/(data[1]-data[2])),
        Vec2::new((data[3]-target)/(data[3]-data[2]), 1.0),
        Vec2::new(0.0, (data[0]-target)/(data[0]-data[3])),   
    ];           
    let mut lines = Vec::new();
    for i in lookup[case].iter() {
        lines.push(verts[*i] + offset);
    }
    
    lines
}
pub fn march(data: ArrayView2<Cell>, target: f32, mat_lookup: &HashMap<GroundMaterial, f32>) -> Vec<GVert> {
    let before = std::time::Instant::now();

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
    let mut mesh: Vec<GVert> = Vec::new();       
    
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
            let corners: [f32; 4] = [
                                        data.get((x,     y))    .unwrap(), 
                                        data.get((x + 1, y))    .unwrap(), 
                                        data.get((x + 1, y + 1)).unwrap(),
                                        data.get((x,     y + 1)).unwrap()
                                     ].map(|c| *mat_lookup.get(&c.material).unwrap());
            
            for id in lookup[case].iter() {
                let pos = verts[*id] + offset;
                mesh.push(gvert(pos, corners));
            }            
        }   
    }
    //println!("\n{:?}", std::time::Instant::now() - before);
    mesh
}
