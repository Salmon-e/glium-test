use glam::*;

// Corrects overlap between a movable aabb and a stationary segment given the velocity of the aabb before collision
pub fn correct_aabb_segment(aabb: (Vec2, Vec2), line: (Vec2, Vec2), vel: Vec2) {

}
pub const EPSILON: f32 = 0.00001;
pub fn intersection(line1: (Vec2, Vec2), line2: (Vec2, Vec2)) -> Option<Vec2> {
    let (v1, v2) = line1;
    let (v3, v4) = line2;
    let d = (v1.x - v2.x) * (v3.y-v4.y) - (v1.y-v2.y) * (v3.x - v4.x);       
    if d.abs() < EPSILON {return None;}
    let ix = ((v1.x * v2.y - v1.y * v2.x) * (v3.x - v4.x) - (v1.x - v2.x) * (v3.x * v4.y - v3.y * v4.x))/d;
    let iy = ((v1.x * v2.y - v1.y * v2.x) * (v3.y - v4.y) - (v1.y - v2.y) * (v3.x * v4.y - v3.y * v4.x))/d;   
                    
    Some(Vec2::new(ix, iy))
} 