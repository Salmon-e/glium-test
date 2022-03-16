use glium::glutin::event::*;
use hecs::*;
use crate::*;
pub enum Input {
    Key(KeyboardInput),
    Mouse {
        button: MouseButton,
        state: ElementState
    }
}

pub struct PlayerController {
    pub key_handler: fn(&Input, Vec2, Entity, &mut Ecs) -> bool,
    pub inputs: Vec<(Input, u32)>
}
impl PlayerController {
    pub fn new() -> Self {
        PlayerController {
            key_handler: player_input_handler,
            inputs: Vec::new()
        }
    }
    pub fn handle_inputs(&mut self, mouse_pos: Vec2, e: Entity, ecs: &mut Ecs) {
        let mut to_remove = Vec::new();
        for i in 0..self.inputs.len() {
            let (input, time) = &mut self.inputs[i];            
            if (self.key_handler)(input, mouse_pos, e, ecs) || *time == 0 {
                to_remove.push(i);                
            }
            else {
                *time -= 1;                
            }            
        }
        while !to_remove.is_empty() {
            
            self.inputs.remove(to_remove.pop().unwrap());
        }
    }
}
#[derive(Default)]
pub struct PlayerMovement {
    pub left: bool,
    pub right: bool,
    pub jumping: bool,    
}
pub fn player_input_handler(input: &Input, mouse_pos: Vec2, e: Entity, ecs: &mut Ecs) -> bool {
    let mut to_spawn = Vec::new();
    if let Ok(mut movement) = ecs.entities.get_mut::<PlayerMovement>(e) {
        
        if let Input::Key(key) = *input {
            
            if key.virtual_keycode == Some(VirtualKeyCode::A) {
                movement.left = if key.state == ElementState::Pressed {true} else {false};                
                return true;
            }
            if key.virtual_keycode == Some(VirtualKeyCode::D) {
                movement.right = if key.state == ElementState::Pressed {true} else {false};
                return true;
            }
            if key.virtual_keycode == Some(VirtualKeyCode::W) {
                movement.jumping = if key.state == ElementState::Pressed {true} else {false};
                return true;
            }            
        }
        if let Input::Mouse {
            button, state
        } = *input {
            if button == MouseButton::Left && state == ElementState::Pressed {
                let pos = ecs.entities.get::<Position>(e).unwrap();
                let vel = (mouse_pos - pos.0).normalize()/2.0;
                to_spawn.push((Position(pos.0), Velocity(vel), Explosive, Sprite(vec2(0.5, 0.5))));                
            }
        }
    }    
    for e in to_spawn {
        ecs.entities.spawn(e);
        return true;
    }
    false
}