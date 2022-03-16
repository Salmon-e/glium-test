
use crate::ecs::*;
use glam::*;
use crate::*;
pub type RenderFunc = fn(Vec<usize>, &Ecs, Vec2, &Display, &mut Frame, &Shaders, &mut BufferStorage);
// Copy paste: (entities: Vec<usize>, ecs: &ECS, camera_pos: Vec2, display: &Display, target: &mut Frame, shaders: &Shaders, buffer_storage: &BufferStorage)
pub struct BufferStorage {
    pub sprite_buffer: VertexBuffer<Vert>
}
// Filter: Position, Sprite
pub fn sprite_render(entities: Vec<usize>, ecs: &Ecs, camera_pos: Vec2, display: &Display, target: &mut Frame, shaders: &Shaders, buffer_storage: &mut BufferStorage) {           
    if entities.is_empty() {return}
    let mut mesh = Vec::new();
    for entity in entities {
        let Position(pos) = ecs.get_component::<Position>(entity).unwrap();
        let Sprite(size) = ecs.get_component::<Sprite>(entity).unwrap();
        let sizex = vec2(size.x, 0.0);
        let sizey = vec2(0.0, size.y);
        mesh.push(vert(*pos));
        mesh.push(vert(*pos + sizex));
        mesh.push(vert(*pos + sizey));
        mesh.push(vert(*pos + *size));
        mesh.push(vert(*pos + sizex));
        mesh.push(vert(*pos + sizey));
    }
    
    buffer_storage.sprite_buffer = VertexBuffer::new(display, &mesh).unwrap();
    
    
    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);
    let (width, height) = display.get_framebuffer_dimensions();
    target.draw(&buffer_storage.sprite_buffer, &indices, &shaders.default, &uniform!{
        resolution: (width as f32, height as f32),
        offset: (camera_pos.x, camera_pos.y)
    }, &Default::default()).unwrap();
}