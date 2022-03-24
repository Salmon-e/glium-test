
use std::collections::HashMap;

use crate::{world::*, Shaders};

use glam::*;
use glium::{*, uniforms::*};
use glium::buffer::*;
use ndarray::Array2;
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct GVert {
    pub position: [f32; 2],
    _pad: [u32; 2],
    pub corners: [f32; 4]
}
pub fn gvert(vec: Vec2, corners: [f32; 4]) -> GVert {
    GVert {
        position: [vec.x, vec.y],
        _pad: [0, 0],
        corners
    }
}
implement_uniform_block!(GVert, position, corners);
implement_vertex!(GVert, position, corners);
#[derive(Copy, Clone)]
pub struct CellDat {
    pub value: f32,
    pub texture: f32
}

implement_uniform_block!(CellDat, value, texture);
fn celldat(v: f32, t: f32) -> CellDat {
    CellDat{value: v, texture: t}
}
pub struct WorldMesh {   
    pub stored_buffers: Vec<((i32, i32), VertexBuffer<GVert>)>,    
    pub mat_lookup: HashMap<GroundMaterial, f32>
}
pub struct ChunkMesh {
    position: (i32, i32),
    verts: Vec<GVert>
}
impl ChunkMesh {
    pub fn new(position: (i32, i32), verts: Vec<GVert>) -> ChunkMesh{
        ChunkMesh {
            position,
            verts
        }
    }
}
impl WorldMesh {
    pub fn new(mat_lookup: HashMap<GroundMaterial, f32>) -> WorldMesh {   
        WorldMesh {            
            stored_buffers: Vec::new(),
            mat_lookup        
        }
    }
    pub fn create_vertex_buffer(&self, shaders: &Shaders, display: &Display, data: Array2<Cell>) -> VertexBuffer<GVert> {

        let mut cell_data = [celldat(0.0, 0.0); (CHUNK_SIZE.0 + 1) * (CHUNK_SIZE.1 + 1)];
        for x in 0..CHUNK_SIZE.0 + 1 {
            for y in 0..CHUNK_SIZE.1 + 1 {
                let i = y * (CHUNK_SIZE.0 + 1) + x;
                let cell = *data.get((x, y)).unwrap();
                cell_data[i] = celldat(cell.value, self.mat_lookup[&cell.material]);
            }
        }    
        

        let cell_buffer = Buffer::new(display, cell_data.as_slice(), BufferType::UniformBuffer, BufferMode::Default).unwrap();
        let vert_buffer: VertexBuffer<GVert> = VertexBuffer::empty_dynamic(display, CHUNK_SIZE.0 * CHUNK_SIZE.1 * 12).unwrap();
       
        
        shaders.square_march.execute(uniform! {
            Cells: &cell_buffer,
            outb: &*vert_buffer
        }, 1, 1, 1);
        vert_buffer
    }
    pub fn update_meshes(&mut self, display: &Display, shaders: &Shaders, world: &World) {   
        let before = std::time::Instant::now();

        for (chunk_pos, chunk) in world.chunks.iter() {
            if std::time::Instant::now() - before > std::time::Duration::from_millis(3) {break}
            if !*chunk.remesh_queued.borrow() {
                continue
            }
            *chunk.remesh_queued.borrow_mut() = false; 

            let updated_buffer = self.create_vertex_buffer(shaders, display, world.get_chunk_cell_arr(*chunk_pos));
           
            let mut i = 0;   
            let mut replaced = false;         
            for (pos, _) in self.stored_buffers.iter_mut() {
                if *pos == *chunk_pos {  
                    replaced = true;                  
                    break;
                }              
                i += 1; 
            }
            if replaced {
                self.stored_buffers[i].1 = updated_buffer;
            }
            else {
                self.stored_buffers.push((*chunk_pos, updated_buffer));
            }            
        }        
    }
    pub fn free_buffer(&mut self, chunk_pos: (i32, i32)) {
        let mut to_free = Vec::new();
        let mut i: usize = 0;
        for (pos, _) in self.stored_buffers.iter_mut() {
            if chunk_pos == *pos {
                to_free.push(i);
            }
            i += 1;
        }
        for index in to_free {
            self.stored_buffers.remove(index);
        }
    }
}