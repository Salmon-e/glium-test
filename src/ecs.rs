use std::any::Any;
use glium::*;
use glam::*;
use crate::world::*;
use crate::renderers::*;
use crate::*;
use crate::impl_component;
pub struct Ecs {
    entities: Vec<Entity>,
    systems: Vec<System>,
    renderers: Vec<Renderer>,
    holes: Vec<usize>
}
// Entity id, time since last frame as a fraction of a frame at 60fps, the entity system, and the world
type SystemFunc = fn(usize, f32, &mut Ecs, &mut World) -> ();

pub struct System {
    pub func: SystemFunc,
    pub filter: Vec<usize>
}
pub struct Renderer {
    pub draw: RenderFunc,
    pub filter: Vec<usize>
}
pub struct Entity {
    components: [Option<Box<dyn Component>>; C_COUNT],
    alive: bool
}
pub const C_COUNT: usize = 16;
pub trait Component: 'static {
    fn index() -> usize where Self : Sized;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
impl Ecs {
    pub fn new() -> Ecs {
        Ecs {
            entities: Vec::new(),
            systems: Vec::new(),
            renderers: Vec::new(),
            holes: Vec::new()
        }
    }
    pub fn get_component<T: Component>(&self, entity: usize) -> Option<&T> {
        let component_option = self.entities[entity].components[T::index()].as_ref();
        if let Some(component) = component_option {
            if let Some(c) = component.as_any().downcast_ref::<T>() {
                return Some(c)
            }            
        }
        None        
    }
    pub fn get_component_mut<T: Component>(&mut self, entity: usize) -> Option<&mut T> {
        let component_option = self.entities[entity].components[T::index()].as_mut();
        if let Some(component) = component_option {
            if let Some(c) = component.as_any_mut().downcast_mut::<T>() {
                return Some(c)
            }            
        }
        None        
    }
    pub fn remove_component<T: Component>(&mut self, entity: usize) {
        self.entities[entity].components[T::index()] = None;
    } 
    pub fn set_component<T: Component>(&mut self, entity: usize, component: T) {
        self.entities[entity].components[T::index()] = Some(Box::new(component));
    }
    pub fn add_entity(&mut self) -> usize { 
        let new = Entity::new();
        if !self.holes.is_empty() {
            let index = self.holes.pop().unwrap();
            self.entities[index] = new;
            return index;
        }
        self.entities.push(new);
        self.entities.len() - 1
    }
    pub fn remove_entity(&mut self, entity: usize) {
        self.entities[entity].alive = false;
        self.holes.push(entity);
    }
    pub fn is_match(&self, entity: usize, components: &[usize]) -> bool {
        let mut m = true;
        for component in components {
            m &= self.entities[entity].components[*component].is_some();
        }
        m
    }
    pub fn add_system(&mut self, system: System) {
        self.systems.push(system);
    }
    pub fn update(&mut self, world: &mut World, frame_portion: f32) {
        for entity in 0..self.entities.len() {
            for system_i in 0..self.systems.len() {                
                let system = &self.systems[system_i];                
                if self.is_match(entity, &system.filter) && self.entities[entity].alive {
                    (system.func)(entity, frame_portion, self, world)
                }
            }
        }
    }
    pub fn render(&self, camera_pos: Vec2, display: &Display, target: &mut Frame, shaders: &Shaders, buffer_storage: &mut BufferStorage) {
        for render_i in 0..self.systems.len() {                
            let render = &self.renderers[render_i];      
            let mut entities = Vec::new();
            for entity in 0..self.entities.len() {
                if self.is_match(entity, &render.filter) && self.entities[entity].alive {
                    entities.push(entity);
                }
            }            
            (render.draw)(entities, self, camera_pos, display, target, shaders, buffer_storage)
        }
        
    }
    pub fn add_renderer(&mut self, r: Renderer) {
        self.renderers.push(r);
    }
}
impl Entity {
    pub fn new() -> Entity {
        Entity {
            components: [(); C_COUNT].map(|()| None),
            alive: true
        }
    }
}
impl System {
    pub fn new(filter: Vec<usize>, func: SystemFunc) -> System {
        System {
            func, filter
        }
    }
}
impl Renderer {
    pub fn new(filter: Vec<usize>, draw: RenderFunc) -> Renderer {
        Renderer {
            draw, filter
        }
    }
}
pub struct Position(pub Vec2);
pub struct Velocity(pub Vec2);
pub struct Sprite(pub Vec2);
impl_component!(Position, 0);
impl_component!(Velocity, 1);
impl_component!(Sprite, 2);
 