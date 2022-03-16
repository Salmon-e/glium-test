use glium::{self, *, glutin::{*, event::*}};
use glam::*;
use crate::world::*;
use crate::world_mesh::*;
use crate::ecs::*;
mod square_march;
mod world;
mod world_mesh;
mod math;
mod macros;
mod renderers;
mod ecs;
#[derive(Copy, Clone)]
pub struct Vert {
    position: [f32; 2],
    color: [f32; 3]
}
pub fn vert(vec: Vec2) -> Vert {
    Vert {
        position: [vec.x, vec.y],
        color: [1.0, 1.0, 0.0]
    }
}
pub struct Shaders {
    default: Program
}
impl Shaders {
    pub fn new(display: &Display) -> Shaders {
        let vertex_shader_src = include_str!("vert.glsl");
        let fragment_shader_src = include_str!("frag.glsl");
        let default = Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
        .expect("Shaders failed to compile");
        Shaders {
            default
        }
    }
}
implement_vertex!(Vert, position, color);
fn main() {
    let event_loop = event_loop::EventLoop::new();
    let window_builder = window::WindowBuilder::new()
        .with_inner_size(dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Thing ig");
    let context_builder = ContextBuilder::new();
    
    let display = Display::new(window_builder, context_builder, &event_loop).expect("Display creation failed");
    let mut world = World::new();    
    let mut world_mesh = WorldMesh::new();

    let mut ecs = Ecs::new();
    let e = ecs.add_entity();
    ecs.set_component(e, Position(vec2(0.0, 0.0)));
    ecs.set_component(e, Velocity(vec2(0.05, 0.05)));
    ecs.set_component(e, Sprite(vec2(3.0, 3.0)));
    ecs.add_system(System::new(vec![Position::index(), Velocity::index()], |e, t, system, world| {
        let Position(pos) = *system.get_component::<Position>(e).unwrap();
        let Velocity(vel) = *system.get_component::<Velocity>(e).unwrap();
    
        system.set_component(e, Position(pos + vel));
    }));
    ecs.add_renderer(Renderer::new(vec![Position::index(), Sprite::index()], renderers::sprite_render));
    let mut z: f32 = 0.0;
    
    for x in 0..100 {
        for y in 0..99 {    
            world.set_cell(x, y, Cell::new(z-0.5));    
            z = 1.0 - z; 
        }
    }   
    let mut buffer_storage = renderers::BufferStorage {
        sprite_buffer: VertexBuffer::empty_dynamic(&display, 32).unwrap()
    };
    
    
    
    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);
    let shaders= Shaders::new(&display);
    
    
    
    let (width, height) = display.get_framebuffer_dimensions();
    let mut camera_pos = Vec2::new(0.0, 0.0);
    let mut last_frame = std::time::Instant::now();
    
    event_loop.run(move |event, _, control_flow| {        
                
        match event  {
            Event::WindowEvent{event: ev,..} => match ev { 
                WindowEvent::CloseRequested => {
                    
                    *control_flow = event_loop::ControlFlow::Exit;
                    return;
                }                
                WindowEvent::MouseInput {button: b, ..}=> {
                    if b == MouseButton::Left {                      
                        return;
                    }
                }
                WindowEvent::CursorMoved{position: pos, ..} => {
                    camera_pos.x = pos.x as f32/ 10.0 - 120.0;
                    camera_pos.y = pos.y as f32/ 10.0 - 120.0;
                    return;
                }
            _ => return
            }
            
            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => (),
                StartCause::Init => (),
                _ => return,
            },
            _ => return      
        }
        
        
        
        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);        
        *control_flow = event_loop::ControlFlow::WaitUntil(next_frame_time);
        
        let mut target = display.draw();
        
        
        target.clear_color(1.0, 0.5, 1.0, 1.0);
        world_mesh.update_meshes(&display, &world);
        for (pos, buffer) in world_mesh.stored_buffers.iter() {            
            target.draw(buffer, &indices, &shaders.default, &uniform!{
                resolution: (width as f32, height as f32), scale: 30f32, 
                offset: (camera_pos.x + pos.0 as f32 * CHUNK_SIZE.0 as f32, camera_pos.y + pos.1 as f32 * CHUNK_SIZE.1 as f32)
            },&Default::default()).unwrap();
        }
        
        ecs.update(&mut world, 1.0);
        ecs.render(camera_pos, &display, &mut target, &shaders, &mut buffer_storage);
        println!("{:?}", ecs.get_component::<Position>(e).unwrap().0);
        target.finish().unwrap();    
        let frame_time = std::time::Instant::now() - last_frame;
        print!("\r{0} fps        ", 1.0/frame_time.as_secs_f32());
        last_frame = std::time::Instant::now();    
    });
}
