use crate::{config, world::World};
use macroquad::prelude::*;

pub fn draw_world(world: &World) {
    clear_background(BLACK);

    draw_particles(&world);

    draw_rectangle_lines(
        5.0,
        5.0,
        world.bounds.x,
        world.bounds.y,
        config::BOUNDS_THICKNESS,
        config::BOUNDS_COLOR,
    );

    draw_debug(config::PARTICLE_COUNT);
}

fn draw_debug(particle_count: usize) {
    draw_text(
        &format!("Particles: {} | FPS: {}", particle_count, get_fps()),
        10.0,
        20.0,
        20.0,
        WHITE,
    );
}

fn draw_particles(world: &World) {
    for particle in &world.particles {
        draw_circle(
            particle.position.x,
            particle.position.y,
            config::RADIUS,
            config::PARTICLECOLOR,
        );
    }
}
