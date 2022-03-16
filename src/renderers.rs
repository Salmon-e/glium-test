
use crate::ecs::*;
use glam::*;
use crate::*;

#[derive(Copy, Clone)]
pub struct EntityVert {
    position: [f32; 2],
    color: [f32; 3]
}
pub fn vert(vec: Vec2, c: f32) -> EntityVert {
    EntityVert {
        position: [vec.x, vec.y],
        color: [1.0, 1.0, c]
    }
}
implement_vertex!(EntityVert, position, color);
pub struct BufferStorage {
    pub sprite_buffer: VertexBuffer<EntityVert>
}
pub fn render_entities(ecs: &mut Ecs, camera_pos: Vec2, display: &Display, target: &mut Frame, shaders: &Shaders, buffer_storage: &mut BufferStorage) {
    sprite_render(ecs, camera_pos, display, target, shaders, buffer_storage);
}
pub fn sprite_render(ecs: &mut Ecs, camera_pos: Vec2, display: &Display, target: &mut Frame, shaders: &Shaders, buffer_storage: &mut BufferStorage) {           
    let mut mesh = Vec::new();
    for (id, (position, sprite)) in ecs.entities.query_mut::<(&mut Position, &mut Sprite)>() {
        let Position(pos) = position;
        let Sprite(size) = sprite;
        let sizex = vec2(size.x, 0.0);
        let sizey = vec2(0.0, size.y);
        let Color(c) = Color(0.0);
        mesh.push(vert(*pos, c));
        mesh.push(vert(*pos + sizex, c));
        mesh.push(vert(*pos + sizey, c));
        mesh.push(vert(*pos + *size, c));
        mesh.push(vert(*pos + sizex, c));
        mesh.push(vert(*pos + sizey, c));
    }
    
    buffer_storage.sprite_buffer = VertexBuffer::new(display, &mesh).unwrap();
    
    
    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);
    let (width, height) = display.get_framebuffer_dimensions();
    
    target.draw(&buffer_storage.sprite_buffer, &indices, &shaders.entity, &uniform!{
        resolution: (width as f32, height as f32),
        offset: (-camera_pos.x, -camera_pos.y),
        scale: 30f32
    }, &Default::default()).unwrap();
}