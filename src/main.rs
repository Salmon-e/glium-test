use std::time::Instant;

use glium::{self, *, glutin::{*, event::*}, uniforms::{SamplerWrapFunction, MinifySamplerFilter, MagnifySamplerFilter, UniformBuffer}, program::ComputeShader};
use glam::*;
use glium::buffer::*;
use crate::world::*;
use crate::world_mesh::*;
use crate::ecs::*;
use crate::controller::*;
use rand::random;
use noise::{NoiseFn, Perlin, Seedable};
mod square_march;
mod world;
mod world_mesh;
mod math;
#[macro_use]
mod macros;
mod controller;
mod renderers;
mod ecs;
mod systems;
mod texture_loader;

pub struct Shaders {
    terrain: Program,
    entity: Program,
    square_march: ComputeShader
}
impl Shaders {
    pub fn new(display: &Display) -> Shaders {
        let vertex_shader_src = include_str!("../shaders/ter_vert.glsl");
        let fragment_shader_src = include_str!("../shaders/ter_frag.glsl");
        let terrain = Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
        .expect("Shaders failed to compile");
        let vertex_shader_src = include_str!("../shaders/e_vert.glsl");
        let fragment_shader_src = include_str!("../shaders/e_frag.glsl");
        let entity = Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
        .expect("Shaders failed to compile");
        let square_source = include_str!("../shaders/square_march.glsl");
        let square_march = ComputeShader::from_source(display, square_source).expect("Compute shaders failed to compile");
        Shaders {
            terrain,
            entity,
            square_march
        }
    }
    
}

fn main() {
    

    let event_loop = event_loop::EventLoop::new();
    let window_builder = window::WindowBuilder::new()
        .with_inner_size(dpi::LogicalSize::new(1024.0, 768.0))
        .with_title("Swirly world");
    let context_builder = ContextBuilder::new().with_vsync(true);

    let display = Display::new(window_builder, context_builder, &event_loop).expect("Display creation failed");
    let mut world = World::new();    
    let (ground_tex, mat_lookup) = texture_loader::gen_terrain_textures(&display);

    let mut world_mesh = WorldMesh::new(mat_lookup);

    let mut ecs = ecs::Ecs::new();
    let e = ecs.entities.spawn((Position(vec2(30.0, -50.0)), 
                                                Sprite(vec2(2.0, 2.0)), 
                                                Velocity(vec2(0.0, 0.0)), 
                                                Aabb{offset: vec2(0.0, 0.0), size: vec2(2.0, 2.0)},
                                                PlayerMovement::default(),
                                                Grounded(0),
                                                Gravity(vec2(0.0, 0.02))));

    
    
       
    let mut buffer_storage = renderers::BufferStorage {
        sprite_buffer: VertexBuffer::empty_dynamic(&display, 32).unwrap()
    };
    
    
    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);
    let shaders= Shaders::new(&display);
    // Empty buffer for testing
    let junk_buffer = UniformBuffer::new(&display, [[0.0f32, 0.0f32]; 1024]).expect("Buffer failed to create");
    
    let mut out: Buffer<[GVert]> = Buffer::empty_array(&display, BufferType::ShaderStorageBuffer, 12288, BufferMode::Dynamic).unwrap();
    
    shaders.square_march.execute(uniform!{
        Cells: &junk_buffer,
        outb: &out
    }, 1, 1, 1);
    println!("{:?}", &out.map_read()[0]);
    let target_fps = 60.0;
    let mut controller = PlayerController::new();

    
    
    let mut mouse_pos = vec2(0.0, 0.0);
    let mut last_frame = std::time::Instant::now();
    let mut mouse_down = false;
    event_loop.run(move |event, _, control_flow| {        
                
        match event  {
            Event::WindowEvent{event: ev,..} => match ev { 
                WindowEvent::CloseRequested => {
                    
                    *control_flow = event_loop::ControlFlow::Exit;
                    return;
                }                
                WindowEvent::MouseInput {button: b, state: s, ..}=> {
                    controller.inputs.push((Input::Mouse {button: b, state: s}, 3));      
                    if s == ElementState::Pressed && b == MouseButton::Right {
                        mouse_down = true;
                    }    
                    if s == ElementState::Released && b == MouseButton::Right {
                        mouse_down = false;
                    }            
                }
                WindowEvent::CursorMoved{position: pos, ..} => {
                    mouse_pos = vec2(pos.x as f32, pos.y as f32);
                    return;
                },        
                WindowEvent::KeyboardInput{device_id, input, is_synthetic} => {
                    controller.inputs.push((Input::Key(input), 5));                    
                    return;
                }
                _ => return
            }
            
            Event::RedrawRequested(_) => (),
            
            Event::NewEvents(cause) => match cause {
                StartCause::ResumeTimeReached { .. } => (),
                StartCause::Init => (),
                StartCause::Poll => (),
                _ => return,
            },            
            _ => return
        }
        
        
        *control_flow = event_loop::ControlFlow::Poll;
        let camera_pos = ecs.entities.get::<Position>(e).unwrap().0;
        let (camera_chunk_x, camera_chunk_y, _, _) = chunk_cell(camera_pos.x as i32, camera_pos.y as i32);
        for x in -3..3 {
            for y in -3..3 {    
                world.load_chunk((x + camera_chunk_x, y + camera_chunk_y));          
            }
        }
        
        
        
        let mut to_unload = Vec::new();
        for ((x, y), _) in world.chunks.iter() {
            let pv = vec2(*x as f32, *y as f32);
            let cv = vec2(camera_chunk_x as f32, camera_chunk_y as f32);
            if pv.distance(cv) > 5.0 {
                to_unload.push((*x, *y));
            }
        }
        for c in to_unload {world.unload_chunk(&mut world_mesh, c)}
        let mut target = display.draw();        
        let (width, height) = display.get_framebuffer_dimensions();
        target.clear_color(0.5, 0.5,1.0, 1.0);
        world_mesh.update_meshes(&display, &world);
        for (pos, buffer) in world_mesh.stored_buffers.iter() {            
            target.draw(buffer, &indices, &shaders.terrain, &uniform!{
                resolution: (width as f32, height as f32), scale: 30f32, 
                offset: (-camera_pos.x + pos.0 as f32 * CHUNK_SIZE.0 as f32, -camera_pos.y + pos.1 as f32 * CHUNK_SIZE.1 as f32),
                tex: ground_tex.sampled().wrap_function(SamplerWrapFunction::Repeat).minify_filter(MinifySamplerFilter::Nearest).magnify_filter(MagnifySamplerFilter::Nearest)
            },&Default::default()).unwrap();
        }
        let frame_time = std::time::Instant::now() - last_frame;
        let dt = frame_time.as_secs_f32() * target_fps;
        if mouse_down {
            let m = to_world_space(mouse_pos, camera_pos, 30f32, vec2(width as f32, height as f32));
            let mut cell = world.get_cell(m.x as i32, m.y as i32);
            cell.value = (cell.value + 0.1).clamp(0.2, 1.0);
            world.set_cell(m.x as i32, m.y as i32, cell);
        }
        controller.handle_inputs(to_world_space(mouse_pos, camera_pos, 30f32, vec2(width as f32, height as f32)), e, &mut ecs);
        systems::apply_systems(dt, &mut ecs, &mut world);
        renderers::render_entities(&mut ecs, camera_pos, &display, &mut target, &shaders, &mut buffer_storage);        
        target.finish().unwrap();    
        print!("\r{0} fps        {1} dt {2} chunks", 1.0/frame_time.as_secs_f32(), dt, world.chunks.values().len());
        last_frame = std::time::Instant::now();    
        
    });
}
pub fn to_world_space(pos: Vec2, camera_pos: Vec2, scale: f32, scrn_size: Vec2) -> Vec2 {
    (pos-scrn_size/2.0) / scale * 2.0 + camera_pos
}
