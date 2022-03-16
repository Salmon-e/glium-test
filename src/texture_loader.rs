use std::{collections::HashMap, io::*, fs::File};
use crate::world::GroundMaterial::*;
use glium::{*, texture::*};
use crate::*;
use image;

pub fn gen_terrain_textures(display: &Display) -> (SrgbTexture3d, HashMap<GroundMaterial, f32>) {       
    let pairs = vec![
        (Stone, "stone.png"),
        (Brick, "bricks.png"),
        (Dirt, "dirt.png"),
    ];
    let mut lookup = HashMap::new();
    let mut images = Vec::new();
    let mut i: f32 = 0.0;
    for (material, path) in pairs.iter() {
        let path = "textures/".to_owned() + path;
        let mut file = File::open(path).expect("Could not find file at given path");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Reading an image file failed");
        let image = image::load(Cursor::new(buffer),
        image::ImageFormat::Png).unwrap().to_rgba8();
        let dim = image.dimensions();
        let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dim);        
        images.push(image);
        lookup.insert(*material, i/pairs.len() as f32);
        i += 1.0001;
    }
    let im3d = RawImage3d::from_vec_raw2d(&images);
    let tex3d = SrgbTexture3d::new(display, im3d).expect("Loading raw 2d images into texture failed");
    (tex3d, lookup)
}