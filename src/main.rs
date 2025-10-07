mod config;
mod grid;
mod particle;
mod physics;
mod render;
mod world;

use macroquad::prelude::*;

#[macroquad::main("Physics")]
async fn main() {
    let bounds = Vec2::new(screen_width() - 10.0, screen_height() - 10.0);
    let mut world = world::World::new(bounds);

    loop {
        world.run_simulation_step();
        world.draw();

        next_frame().await;
    }
}
