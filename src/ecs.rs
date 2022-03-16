
use glam::*;
use hecs;

pub struct Ecs {
    pub entities: hecs::World
}
// Entity id, time since last frame as a fraction of a frame at 60fps, the entity system, and the world
impl Ecs {
    pub fn new() -> Self {
        Ecs {
            entities: hecs::World::new()
        }
    }
}




pub struct Position(pub Vec2);
pub struct Velocity(pub Vec2);
pub struct Sprite(pub Vec2);
pub struct Color(pub f32);
pub struct Aabb {
    pub offset: Vec2,
    pub size: Vec2
}
pub struct Explosive;
pub struct Grounded(pub i32);
pub struct Gravity(pub Vec2);

 