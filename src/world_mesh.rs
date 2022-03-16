
use crate::world::*;
use crate::Vert;
use glium::*;
pub struct WorldMesh {   
    pub stored_buffers: Vec<((i32, i32), VertexBuffer<Vert>)>,    
}
pub struct ChunkMesh {
    position: (i32, i32),
    verts: Vec<Vert>
}
impl ChunkMesh {
    pub fn new(position: (i32, i32), verts: Vec<Vert>) -> ChunkMesh{
        ChunkMesh {
            position,
            verts
        }
    }
}
impl WorldMesh {
    pub fn new() -> WorldMesh {   
        WorldMesh {            
            stored_buffers: Vec::new()            
        }
    }

    pub fn update_meshes(&mut self, display: &Display, world: &World) {        
        for (chunk_pos, chunk) in world.chunks.iter() {
            if !*chunk.remesh_queued.borrow() {
                continue
            }
            *chunk.remesh_queued.borrow_mut() = false;
            let mesh = ChunkMesh::new(*chunk_pos, world.get_mesh(*chunk_pos));
            let updated_buffer = VertexBuffer::new(display, &mesh.verts).unwrap();
            let mut i = 0;   
            let mut replaced = false;         
            for (pos, _) in self.stored_buffers.iter_mut() {
                if *pos == mesh.position {  
                    replaced = true;                  
                    break;
                }              
                i += 1; 
            }
            if replaced {
                self.stored_buffers[i].1 = updated_buffer;
            }
            else {
                self.stored_buffers.push((mesh.position, updated_buffer));
            }            
        }
    }
}