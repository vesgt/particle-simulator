use macroquad::prelude::*;

const DOWN: Vec2 = Vec2::new(0.0, 1.0);
const GRAVITY: Vec2 = Vec2::new(0.0, 500.0);
const RADIUS: f32 = 7.5;
const DRAG: f32 = 0.9;
const BOUNCE: f32 = 0.8;

#[macroquad::main("Physics")]
async fn main() {
    let mut pos = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
    let mut velocity = Vec2::new(0.0, 0.0);
    let bounds = Vec2::new(screen_width() - 10.0, screen_height() - 10.0);

    loop {
        clear_background(BLACK);

        apply_gravity(&mut pos, &mut velocity);
        apply_bounds(bounds, &mut pos, &mut velocity);

        draw_rectangle_lines(5.0, 5.0, bounds.x, bounds.y, 2.0, YELLOW);
        draw_circle(pos.x, pos.y, RADIUS, BLUE);

        next_frame().await
    }
}

fn apply_gravity(pos: &mut Vec2, velocity: &mut Vec2) {
    *velocity += DOWN * GRAVITY * get_frame_time() * DRAG; 
    *pos += *velocity * get_frame_time();
}

fn apply_bounds(bounds: Vec2, pos: &mut Vec2, velocity: &mut Vec2) {
    if bounds.y < pos.y + RADIUS {
        pos.y = bounds.y - RADIUS;
        *velocity *= -BOUNCE;
    }
}
