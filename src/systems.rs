use crate::world::*;
use crate::ecs::*;
use crate::*;
use glam::*;
use crate::math::*;


pub fn apply_systems(dt: f32, ecs: &mut Ecs, world: &mut World) {
    default_movement(dt, ecs, world);
    gravity(dt, ecs, world);
    aabb_movement(dt, ecs, world);
    player_movement(dt, ecs, world);
    grounded_tick(dt, ecs, world);
    explode(dt, ecs, world)
}
pub fn default_movement(dt: f32, ecs: &mut Ecs, _world: &mut World) {
        let mut dont_move = Vec::new();
        for (id, _) in ecs.entities.query_mut::<&mut Aabb>() {
            dont_move.push(id);
        }
        for (id, (Position(pos), Velocity(vel))) in ecs.entities.query_mut::<(&mut Position, &mut Velocity)>() {
            if !dont_move.contains(&id) {
                *pos += *vel;
            }
        }
}
pub fn gravity(dt: f32, ecs: &mut Ecs, _world: &mut World) {
    for (id, (Velocity(vel), Gravity(gravity))) in ecs.entities.query_mut::<(&mut Velocity, &mut Gravity)>() {
        *vel += *gravity * dt;
    }

}
pub fn player_movement(_dt: f32, ecs: &mut Ecs, _world: &mut World) {
    for (_, (Velocity(vel), movement, Grounded(ground_time))) in ecs.entities.query_mut::<(&mut Velocity, &mut PlayerMovement, &mut Grounded)>() {
        if movement.left {
            vel.x = -0.3;
        }
        if movement.right {
            vel.x = 0.3;
        }
        if !(movement.left || movement.right) {vel.x = 0.0}
        if movement.jumping && *ground_time > 0 {
            vel.y = -0.5;            
        }
        
    }
}
pub fn explode(_: f32, ecs: &mut Ecs, world: &mut World) {
    let mut destroyed = Vec::new();
    for (id, (Position(pos), _)) in ecs.entities.query_mut::<(&mut Position, &mut Explosive)>() {
        if world.get_cell(pos.x as i32, pos.y as i32).value > 0.0 {
            destroyed.push(id);
            for x in (pos.x as i32 - 10)..(pos.x as i32 + 10) {
                for y in (pos.y as i32 - 10)..(pos.y as i32 + 10) {
                    let mut cell = world.get_cell(x as i32, y as i32);   
                    let dist = vec2(x as f32, y as f32).distance(*pos);          
                    if dist < 10.0 {
                        cell.value = cell.value.min(dist/10.0 - 0.5);
                        world.set_cell(x, y, cell);
                    }
                }
            }
        }
    }
    for id in destroyed {
        ecs.entities.despawn(id);
    }
}
pub fn aabb_movement(dt: f32, ecs: &mut Ecs, world: &mut World) {
    let mut grounded = Vec::new();
    for (id, (position, velocity, aabb)) in ecs.entities.query_mut::<(&mut Position, &mut Velocity, &mut Aabb)>() {
        let Position(pos) = position;
        let Velocity(vel) = velocity;
        let Aabb {
            offset, size
        } = aabb;
        *pos += *offset;   
        let pre_pos = *pos;
        
        pos.x += vel.x * dt;
        let intersections = world.get_aabb_intersections(*pos, *size);
        if vel.x > 0.0 && !intersections.is_empty() {
            let (i, line) = intersections.into_iter().reduce(|(ai, al), (i, l)| if ai.x < i.x {(ai, al)} else {(i, l)}).unwrap();
            pos.x = f32::max(pre_pos.x, i.x - size.x) - EPSILON;
            if (line.0.x - line.1.x).abs() > EPSILON {
                let slope = (line.0.y - line.1.y)/(line.0.x - line.1.x);                
                if slope.abs() < 1.5 {
                    vel.y = vel.x * slope;                    
                    if vel.y < 0.0 {
                        grounded.push(id);
                    }                    
                }
            }
            vel.x = 0.0;
        }
        else if vel.x < 0.0 && !intersections.is_empty() {
            let (i, line) = intersections.into_iter().reduce(|(ai, al), (i, l)| if ai.x > i.x {(ai, al)} else {(i, l)}).unwrap();
            pos.x = f32::min(pre_pos.x, i.x) + EPSILON;
            if (line.0.x - line.1.x).abs() > EPSILON {
                let slope = (line.0.y - line.1.y)/(line.0.x - line.1.x);                
                if slope.abs() < 1.5 {
                    vel.y = vel.x * slope;                    
                    if vel.y < 0.0 {
                        grounded.push(id);
                    }                    
                }
            }
            vel.x = 0.0;
        }
        pos.y += vel.y * dt;
        let intersections = world.get_aabb_intersections(*pos, *size);
        if vel.y > 0.0 && !intersections.is_empty() {
            pos.y = f32::max(pre_pos.y, intersections.into_iter().map(|(i, _)|i.y).reduce(|a, i| f32::min(a, i)).unwrap() - size.y) - EPSILON;
            vel.y = 0.0;
            grounded.push(id);
        }
        else if vel.y < 0.0 && !intersections.is_empty() {
            pos.y = f32::min(pre_pos.y, intersections.into_iter().map(|(i, _)|i.y).reduce(|a, i| f32::max(a, i)).unwrap()) + EPSILON;
            vel.y = 0.0;
        }        
        *pos -= *offset;            
    }
    for entity in grounded {
        if let Ok(mut g) = ecs.entities.get_mut::<Grounded>(entity) {
            g.0 = 20;
        }
    }
}
pub fn grounded_tick(_dt: f32, ecs: &mut Ecs, _world: &mut World) {
    for (_, Grounded(time)) in ecs.entities.query_mut::<&mut Grounded>() {
        if *time != 0 {
            *time -= 1;
        }
    }
}