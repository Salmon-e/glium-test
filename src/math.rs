use glam::*;


pub const EPSILON: f32 = 0.0001;
pub fn intersection(line1: (Vec2, Vec2), line2: Line) -> Option<Vec2> {
    let (v1, v2) = line1;
    match line2 {
        Line::Horizontal {
            y, start, end
        } => {
            if !between(y, v1.y, v2.y) || (v1.x - v2.x).abs() < EPSILON {None} else {
                let m = (v1.x - v2.x)/(v1.y - v2.y);
                let min = if v1.y < v2.y {v1} else {v2};
                let i = vec2((y - min.y) * m + min.x, y);
                if between(i.x, start, end) {
                    Some(i)
                }
                else {None} 
            }
        }
        Line::Vertical {
            x, start, end
        } => {
            if !between(x, v1.x, v2.x) || (v1.y - v2.y).abs() < EPSILON {None} else {
                let m = (v1.y - v2.y)/(v1.x - v2.x);
                let min = if v1.x < v2.x {v1} else {v2};
                let i = vec2(x, (x - min.x) * m + min.y);
                if between(i.y, start, end) {
                    Some(i)
                }
                else {None}
            }
        }
    }    
} 
pub fn get_aabb_intersections(line: (Vec2, Vec2), pos: Vec2, size: Vec2) -> Vec<Vec2> {
    let top = Line::Horizontal{y: pos.y, start: pos.x, end: pos.x + size.x};
    let bottom = Line::Horizontal{y: pos.y + size.y, start: pos.x, end: pos.x + size.x};
    let left = Line::Vertical{x: pos.x, start: pos.y, end: pos.y + size.y};
    let right = Line::Vertical{x: pos.x + size.x, start: pos.y, end: pos.y + size.y};

    let mut intersections = Vec::new();
    if let Some(v) = intersection(line, top) {intersections.push(v)}
    if let Some(v) = intersection(line, right) {intersections.push(v)}
    if let Some(v) = intersection(line,bottom) {intersections.push(v)}
    if let Some(v) = intersection(line, left) {intersections.push(v)}
    if point_in_aabb(line.0, pos, size) {intersections.push(line.0)}
    if point_in_aabb(line.1, pos, size) {intersections.push(line.1)}
    intersections
}

pub fn point_in_aabb(point: Vec2, pos: Vec2, size: Vec2) -> bool {
    between(point.x, pos.x, pos.x + size.x) && between(point.y, pos.y, pos.y + size.y)
}
pub enum Line {
    Horizontal {
        y: f32,
        start: f32,
        end: f32
    },
    Vertical {
        x: f32,
        start: f32,
        end: f32
    }
}
pub fn between(x: f32, bound1: f32, bound2: f32) -> bool {
    if bound1 > bound2 {
        return bound2 <= x && x <= bound1
    }
    else {
        return bound1 <= x && x <= bound2
    }
}