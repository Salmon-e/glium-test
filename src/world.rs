use std::collections::HashMap;
use ndarray::*;
use glam::*;
use noise::NoiseFn;
use noise::Seedable;
use crate::square_march;

use crate::math;
use crate::world_mesh::GVert;
use crate::world_mesh::WorldMesh;
pub struct World {
    pub chunks: HashMap<(i32, i32), Chunk>        
}
pub struct Chunk {
    pub cells: Array2<Cell>,
    pub remesh_queued: std::cell::RefCell<bool>
}

impl World {
    
    pub fn get_mesh(&self, chunk_pos: (i32, i32), mat_lookup: &HashMap<GroundMaterial, f32>) -> Vec<GVert> {               
        let arr = self.get_chunk_cell_arr(chunk_pos);
        let slice = arr.slice(s![0..arr.dim().0, 0..arr.dim().1]);
        
        square_march::march(slice, 0.0, mat_lookup)
    }    
    pub fn new() -> World {
        World {
            chunks: HashMap::new()            
        }
    }
    pub fn get_chunk_cell_arr(&self, chunk_pos: (i32, i32)) -> Array2<Cell> {               
        let mut arr = Array::from_elem((CHUNK_SIZE.0 + 1, CHUNK_SIZE.1 + 1), Cell::default());
        for x in 0..(CHUNK_SIZE.0 + 1) {
            for y in 0..(CHUNK_SIZE.1 + 1) {
                    *arr.get_mut((x, y)).unwrap() = self.get_cell(chunk_pos.0 * CHUNK_SIZE_I32.0 + x as i32, chunk_pos.1 * CHUNK_SIZE_I32.1 + y as i32); 
            }
        }
        arr        
    }    
    pub fn get_cell_collision(&self, x: i32, y: i32) -> Vec<(Vec2, Vec2)> {
        let mut lines  = Vec::new();
        let verts = square_march::get_lines([self.get_cell(x, y).value, self.get_cell(x + 1, y).value, self.get_cell(x + 1, y + 1).value, self.get_cell(x, y + 1).value], vec2(x as f32, y as f32), 0.0);
        let mut i = 1;
        while i < verts.len() {
            lines.push((verts[i], verts[i - 1]));
            i += 2;
        }
        lines
    }
    pub fn get_aabb_intersections(&self, pos: Vec2, size: Vec2) -> Vec<(Vec2, (Vec2, Vec2))> {
        let mut intersections = Vec::new(); 
        for x in (pos.x as i32)..((pos.x + size.x) as i32 + 1) {
            for y in (pos.y as i32)..((pos.y + size.y) as i32 + 1) {    
                for line in self.get_cell_collision(x, y).iter() {
                    for i in math::get_aabb_intersections(*line, pos, size) {
                        intersections.push((i, *line));
                    }
                }
            }
        }
        intersections
    }
    pub fn get_cell(&self, x: i32, y: i32) -> Cell {
        let (chunk_x, chunk_y, cell_x, cell_y) = chunk_cell(x, y);

        if let Some(chunk) = self.chunks.get(&(chunk_x, chunk_y)) {
            return *chunk.cells.get((cell_x, cell_y)).unwrap_or(&Cell::default())
        }
        Cell::default()
    }
    pub fn set_cell(&mut self, x: i32, y: i32, value: Cell) {
        
        let (chunk_x, chunk_y, cell_x, cell_y) = chunk_cell(x, y);
        if !self.chunks.contains_key(&(chunk_x, chunk_y)) {
            self.load_chunk((chunk_x, chunk_y))
        }
        if let Some(chunk) = self.chunks.get_mut(&(chunk_x, chunk_y)) {
            if let Some(cell) = chunk.cells.get_mut((cell_x, cell_y)) {
                *cell = value;
                chunk.remesh_queued = std::cell::RefCell::new(true);       
            }     
        }
        let remesh_x = if cell_x == 0 {-1}
            else if cell_x == CHUNK_SIZE.0 - 1 {1}
            else {0};
        let remesh_y = if cell_y == 0 {-1}
            else if cell_y == CHUNK_SIZE.1 - 1 {1}
            else {0};
        for (offset_x, offset_y) in [(remesh_x, 0), (remesh_x, remesh_y), (0, remesh_y)] {
            if (offset_x, offset_y) == (0, 0) {
                continue;
            }
            if let Some(chunk) = self.chunks.get_mut(&(chunk_x + offset_x, chunk_y + offset_y)) {
                chunk.remesh_queued = std::cell::RefCell::new(true);
            }
        }
    }    
    pub fn load_chunk(&mut self, chunk_pos: (i32, i32)) {
        // TODO split into load from file and terrain gen func, for now, just terrain gen
        if self.chunks.contains_key(&chunk_pos) {
            return
        }
        
        for (x, y) in [(1, 0), (0, 1), (1, 1), (1, -1), (-1, 0), (0, -1), (-1, -1), (-1, 1)] {
            if let Some(c) = self.chunks.get(&(chunk_pos.0 + x, chunk_pos.1 + y)) {
                *c.remesh_queued.borrow_mut() = true;
            }
        }
        let noise = noise::OpenSimplex::new();
        let noise2 = noise::OpenSimplex::new();
        let noise3 = noise::OpenSimplex::new();
        let mut chunk = Chunk::new();
        for cell_x in 0..CHUNK_SIZE.0 {
            for cell_y in 0..CHUNK_SIZE.1 {
                let (x, y) = cell_pos(chunk_pos, (cell_x, cell_y));               
                let cell = chunk.cells.get_mut((cell_x, cell_y)).unwrap();
                *cell = Cell::new(noise.get([x as f64 / 20.0, y as f64 / 20.]) as f32);
                let val = noise2.get([x as f64 / 10.0, y as f64 / 10.]);
                let val2 = noise3.get([x as f64 / 10.0, y as f64 / 10.]);
                cell.material = if val > val2 {GroundMaterial::Dirt} else if cell.value > val2 as f32 {GroundMaterial::Stone} else {GroundMaterial::Brick};         
            }
        }
        self.chunks.insert(chunk_pos, chunk);
    }
    pub fn unload_chunk(&mut self, world_mesh: &mut WorldMesh, chunk_pos: (i32, i32)) {
        // TODO: Save the chunk to mem
        self.chunks.remove(&chunk_pos);
        world_mesh.free_buffer(chunk_pos);
    }
}
pub fn chunk_cell(x: i32, y: i32) -> (i32, i32, usize, usize) {
    let mut chunk_x = x/CHUNK_SIZE_I32.0;
        let mut chunk_y = y/CHUNK_SIZE_I32.1;
        if x < 0 {
            chunk_x -= 1;
        }
        if y < 0 {
            chunk_y -= 1;
        }
        let cell_x = (x % CHUNK_SIZE_I32.0).abs() as usize;
        let cell_y = (y % CHUNK_SIZE_I32.1).abs() as usize;
        (chunk_x, chunk_y, cell_x, cell_y)
}
pub fn cell_pos(chunk_pos: (i32, i32), cell_pos: (usize, usize)) -> (i32, i32){
    (chunk_pos.0 * CHUNK_SIZE_I32.0 + cell_pos.0 as i32, chunk_pos.1 * CHUNK_SIZE_I32.1 + cell_pos.1 as i32)
}
pub const CHUNK_SIZE: (usize, usize) = (32, 32);
pub const CHUNK_SIZE_I32: (i32, i32) = (CHUNK_SIZE.0 as i32, CHUNK_SIZE.1 as i32);
impl Chunk {
    pub fn new() -> Chunk {
        Chunk{cells: Array2::from_elem(CHUNK_SIZE, Cell::new(0.0)), remesh_queued: std::cell::RefCell::new(true)}
    }   
}
#[derive(Clone, Copy, Default)]
pub struct Cell {
    pub value: f32,
    pub material: GroundMaterial
}
impl Cell {
    pub fn new(value: f32) -> Cell {
        Cell{value, material: GroundMaterial::Dirt}        
    } 
}
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum GroundMaterial {
    Dirt,
    Stone,
    Brick
}
impl Default for GroundMaterial {
    fn default() -> Self {
        GroundMaterial::Dirt
    }
}
