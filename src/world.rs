use std::collections::HashMap;
use ndarray::*;
use glam::*;
use crate::square_march;
use crate::Vert;


pub struct World {
    pub chunks: HashMap<(i32, i32), Chunk>        
}
pub struct Chunk {
    pub cells: Array2<Cell>,
    pub remesh_queued: std::cell::RefCell<bool>
}
#[derive(Clone, Copy, Default)]
pub struct Cell {
    pub value: f32
}
impl World {
    
    pub fn get_mesh(&self, chunk_pos: (i32, i32)) -> Vec<Vert> {               
        let arr = self.get_chunk_cell_arr(chunk_pos);
        let slice = arr.slice(s![0..arr.dim().0, 0..arr.dim().1]);
        square_march::march(slice, 0.0)
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
    pub fn get_cell(&self, x: i32, y: i32) -> Cell {
        let chunk_x = x/CHUNK_SIZE_I32.0;
        let chunk_y = y/CHUNK_SIZE_I32.1;

        let cell_x = x % CHUNK_SIZE_I32.0;
        let cell_y = y % CHUNK_SIZE_I32.1;

        if let Some(chunk) = self.chunks.get(&(chunk_x, chunk_y)) {
            return *chunk.cells.get((cell_x as usize, cell_y as usize)).unwrap()
        }
        Cell::default()
    }
    pub fn set_cell(&mut self, x: i32, y: i32, value: Cell) {
        let chunk_x = x/CHUNK_SIZE_I32.0;
        let chunk_y = y/CHUNK_SIZE_I32.1;

        let cell_x = (x % CHUNK_SIZE_I32.0) as usize;
        let cell_y = (y % CHUNK_SIZE_I32.1) as usize;

        if let Some(chunk) = self.chunks.get_mut(&(chunk_x, chunk_y)) {
            let cell = chunk.cells.get_mut((cell_x, cell_y)).unwrap();
            *cell = value;
            chunk.remesh_queued = std::cell::RefCell::new(true);            
        }
        else {
            let mut new_chunk = Chunk::new();
            let cell = new_chunk.cells.get_mut((cell_x, cell_y)).unwrap();
            *cell = value;

            // TODO replace with some method for fetching unloaded chunks instead of creating a blank one

            self.chunks.insert((chunk_x, chunk_y), new_chunk);
        }
        let remesh_x = if cell_x == 0 {-1}
            else if cell_x == CHUNK_SIZE.0 {1}
            else {0};
        let remesh_y = if cell_y == 0 {-1}
            else if cell_y == CHUNK_SIZE.1 {1}
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
}
pub const CHUNK_SIZE: (usize, usize) = (32, 32);
pub const CHUNK_SIZE_I32: (i32, i32) = (CHUNK_SIZE.0 as i32, CHUNK_SIZE.1 as i32);
impl Chunk {
    pub fn new() -> Chunk {
        Chunk{cells: Array2::from_elem(CHUNK_SIZE, Cell::new(0.0)), remesh_queued: std::cell::RefCell::new(false)}
    }   
}
impl Cell {
    pub fn new(value: f32) -> Cell {
        Cell{value}
    } 
}