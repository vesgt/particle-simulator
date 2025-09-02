use macroquad::prelude::*;

const DOWN: Vec2 = Vec2::new(0.0, 1.0);
const GRAVITY: Vec2 = Vec2::new(0.0, 150.0);
const RADIUS: f32 = 7.5;
const BOUNCE: f32 = 0.5;
//const VELOCITY_DAMPING: f32 = 0.95;

#[macroquad::main("Physics")]
async fn main() {
    let ball_count = 400;
    let (cols, rows) = calculate_grid_size(ball_count);
    let spacing = 30.0;
    let start_x = screen_width() / 2.0 - (cols as f32 * spacing) / 2.0;
    let start_y = screen_height() / 2.0 - (rows as f32 * spacing) / 2.0;
    let mut positions = Vec::<Vec2>::new();
    let mut velocities = Vec::<Vec2>::new();
    let bounds = Vec2::new(screen_width() - 10.0, screen_height() - 10.0);

    let mut balls_spawned = 0;
    for row in 0..rows {
        for col in 0..cols {
            if balls_spawned >= ball_count {
                break;
            }
            positions.push(Vec2::new(
                start_x + col as f32 * spacing,
                start_y + row as f32 * spacing
            ));
            velocities.push(Vec2::new(0.0, 0.0));
            balls_spawned += 1;
        }

        if balls_spawned >= ball_count {
            break;
        }
    }

    loop {
        clear_background(BLACK);

        push_away(&mut positions, &mut velocities);
        apply_gravities(&mut positions, &mut velocities);
        apply_bounds(bounds, &mut positions, &mut velocities);

        apply_pressure(&mut positions, &mut velocities);

        draw_rectangle_lines(5.0, 5.0, bounds.x, bounds.y, 2.0, YELLOW);
        draw_circles(&positions);

        next_frame().await;
    }
}

fn apply_pressure(positions: &mut Vec<Vec2>, velocities: &mut Vec<Vec2>) {
    let densities = compute_densities(positions);
    let n = positions.len();
    let influence_radius = RADIUS * 3.0;
    let dt = get_frame_time();
    let k = 50.0;           // pressure strength, tune
    let fmax = 500.0;       // per-step force cap, tune
    let eps = 1e-3;

    for i in 0..n {
        let mut force = Vec2::ZERO;

        for j in 0..n {
            if i == j { continue; }
            let dv = positions[i] - positions[j];
            let r = dv.length();
            if r > eps && r < influence_radius {
                let dir = dv / r;
                let weight = 1.0 - r / influence_radius;
                let push = (densities[i] - densities[j]) * weight;
                force += dir * push;
            }
        }

        if force.length() > fmax {
            force = force.normalize() * fmax;
        }

        velocities[i] += force * k * dt;
    }
}

fn compute_densities(positions: &Vec<Vec2>) -> Vec<f32> {
    let n = positions.len();
    let interact_radius = RADIUS * 3.0;
    let mut densities = vec![0.0; n];

    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }
            let r = (positions[i] - positions[j]).length();
            if r < interact_radius {
                densities[i] += 1.0 - (r / interact_radius);
            }
        }
    }
    densities
}

fn push_away(positions: &mut Vec::<Vec2>, velocities: &mut Vec::<Vec2>) {
    for i in 0..positions.len() {
        for j in (i+1)..positions.len() {
            let distance_vector = positions[i] - positions[j];
            let distance = distance_vector.length();
            let min_distance = RADIUS * 2.0;

            if distance < min_distance {
                let overlap = 0.5 * (min_distance - distance);
                let dir = if distance > 0.01 { distance_vector.normalize() } else { Vec2::X };
                positions[i] += dir * overlap;
                positions[j] -= dir * overlap;
            }            
        }
    }
}

fn apply_gravity(pos: &mut Vec2, velocity: &mut Vec2) {
    *velocity += DOWN * GRAVITY * get_frame_time();
    *velocity *= 0.98_f32.powf(get_frame_time() * 60.0);
    if velocity.length() < 1e-2 {
        *velocity = Vec2::ZERO;
    }
    *pos += *velocity * get_frame_time();
}

fn apply_gravities(positions: &mut Vec::<Vec2>, velocities: &mut Vec::<Vec2>) {
    for i in 0..positions.len() {
        apply_gravity(&mut positions[i], &mut velocities[i]);
    }
}

fn apply_bound(bounds: Vec2, pos: &mut Vec2, velocity: &mut Vec2) {
    if bounds.y + 5.0 < pos.y + RADIUS {
        pos.y = bounds.y - RADIUS;
        velocity.y *= -BOUNCE;
    }

    if bounds.x + 5.0 < pos.x + RADIUS {
        pos.x = bounds.x - RADIUS;
        velocity.x *= -BOUNCE;
    }

    if 5.0 > pos.x - RADIUS {
        pos.x = 5.0 + RADIUS;
        velocity.x *= -BOUNCE;
    }

    if 5.0 > pos.y - RADIUS {
        pos.y = 5.0 + RADIUS;
        velocity.y *= -BOUNCE;
    }
}

fn apply_bounds(bounds: Vec2, positions: &mut Vec::<Vec2>, velocities: &mut Vec::<Vec2>) {
    for i in 0..positions.len() {
        apply_bound(bounds, &mut positions[i], &mut velocities[i]);
    }
}

fn draw_circles(positions: &Vec::<Vec2>) {
    for i in 0..positions.len() {
        draw_circle(positions[i].x, positions[i].y, RADIUS, BLUE);
    }
}

fn calculate_grid_size(ball_count: i32) -> (i32, i32) {
    let sqrt_count = (ball_count as f32).sqrt() as i32;
    let cols = sqrt_count;
    let rows = (ball_count + cols - 1) / cols; // Ceiling division
    (cols, rows)
}
