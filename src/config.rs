use macroquad::prelude::*;

pub const DOWN: Vec2 = Vec2::new(0.0, 1.0);
pub const GRAVITY: Vec2 = Vec2::new(0.0, 150.0);
pub const RADIUS: f32 = 7.5;
pub const BOUNCE: f32 = 0.5;
pub const PARTICLECOLOR: Color = BLUE;
pub const BOUNDS_COLOR: Color = YELLOW;
pub const BOUNDS_THICKNESS: f32 = 2.0;
pub const PARTICLE_COUNT: usize = 400;
pub const CELL_SIZE: f32 = 30.0;
