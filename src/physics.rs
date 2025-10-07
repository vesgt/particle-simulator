use crate::config;
use crate::grid::Grid;
use crate::particle::Particle;
use macroquad::prelude::*;

pub fn apply_gravity(particles: &mut [Particle]) {
    for particle in particles {
        particle.velocity += config::DOWN * config::GRAVITY * get_frame_time();
        particle.velocity *= 0.98_f32.powf(get_frame_time() * 60.0);

        if particle.velocity.length() < 1e-2 {
            particle.velocity = Vec2::ZERO;
        }

        particle.position += particle.velocity * get_frame_time();
    }
}

pub fn apply_bounds(particles: &mut [Particle], bounds: Vec2) {
    for particle in particles {
        // Bottom
        if particle.position.y + config::RADIUS > bounds.y + 5.0 {
            particle.position.y = bounds.y + 5.0 - config::RADIUS;
            particle.velocity.y *= -config::BOUNCE;
        }

        // Right
        if particle.position.x + config::RADIUS > bounds.x + 5.0 {
            particle.position.x = bounds.x + 5.0 - config::RADIUS;
            particle.velocity.x *= -config::BOUNCE;
        }

        // Left
        if particle.position.x - config::RADIUS < 5.0 {
            particle.position.x = 5.0 + config::RADIUS;
            particle.velocity.x *= -config::BOUNCE;
        }

        // Top
        if particle.position.y - config::RADIUS < 5.0 {
            particle.position.y = 5.0 + config::RADIUS;
            particle.velocity.y *= -config::BOUNCE;
        }
    }
}

pub fn push_away(particles: &mut [Particle], grid: &Grid) {
    let mut corrections = vec![Vec2::ZERO; particles.len()];

    for i in 0..particles.len() {
        let nearby = grid.get_nearby(particles[i].position);

        for &j in &nearby {
            if i >= j {
                continue;
            } // Avoid checking same pair twice

            let distance_vector = particles[i].position - particles[j].position;
            let distance = distance_vector.length();
            let min_distance = config::RADIUS * 2.0;

            if distance < min_distance && distance > 0.01 {
                let overlap = 0.5 * (min_distance - distance);
                let dir = distance_vector.normalize();

                corrections[i] += dir * overlap;
                corrections[j] -= dir * overlap;
            }
        }
    }

    for i in 0..particles.len() {
        particles[i].position += corrections[i];
    }
}
