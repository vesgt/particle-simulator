use macroquad::prelude::*;

use crate::config;
use crate::grid::Grid;
use crate::particle::Particle;
use crate::physics;
use crate::render;

pub struct World {
    pub bounds: Vec2,
    pub particles: Vec<Particle>,
    grid: Grid,
}

impl World {
    pub fn new(bounds: Vec2) -> Self {
        let particles = Self::create_particles(config::PARTICLE_COUNT, bounds);

        Self {
            bounds,
            particles,
            grid: Grid::new(config::CELL_SIZE),
        }
    }

    pub fn run_simulation_step(&mut self) {
        self.grid.clear();
        for (i, particle) in self.particles.iter().enumerate() {
            self.grid.insert(i, particle.position);
        }

        physics::apply_gravity(&mut self.particles);
        physics::push_away(&mut self.particles, &self.grid);
        // physics::apply_pressure(&self.particles);
        physics::apply_bounds(&mut self.particles, self.bounds);
    }

    pub fn draw(&self) {
        render::draw_world(self);
    }

    pub fn create_particles(particle_count: usize, bounds: Vec2) -> Vec<Particle> {
        let (cols, rows) = Self::calculate_grid_size(particle_count);
        let spacing = 30.0;
        let start_x = bounds.x / 2.0 - (cols as f32 * spacing) / 2.0;
        let start_y = bounds.y / 2.0 - (rows as f32 * spacing) / 2.0;

        let mut particles = Vec::<Particle>::new();
        let mut particles_spawned = 0;

        for row in 0..rows {
            for col in 0..cols {
                if particles_spawned >= particle_count {
                    break;
                }

                let pos = Vec2::new(
                    start_x + col as f32 * spacing,
                    start_y + row as f32 * spacing,
                );

                particles.push(Particle::new(pos, Vec2::new(0.0, 0.0)));
                particles_spawned += 1;
            }

            if particles_spawned >= particle_count {
                break;
            }
        }

        particles
    }

    pub fn calculate_grid_size(particle_count: usize) -> (usize, usize) {
        let sqrt_count = (particle_count as f32).sqrt() as usize;
        let cols = sqrt_count;
        let rows = (particle_count + cols - 1) / cols;
        (cols, rows)
    }
}
