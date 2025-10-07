use macroquad::prelude::*;
use std::{collections::HashMap, i32};

pub struct Grid {
    pub cell_size: f32,
    pub cells: HashMap<(i32, i32), Vec<usize>>,
}

impl Grid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.cells.clear();
    }

    pub fn insert(&mut self, particle_index: usize, particle_pos: Vec2) {
        let cell = self.get_cell(particle_pos);
        self.cells
            .entry(cell)
            .or_insert_with(Vec::new)
            .push(particle_index);
    }

    pub fn get_cell(&self, position: Vec2) -> (i32, i32) {
        (
            (position.x / self.cell_size) as i32,
            (position.y / self.cell_size) as i32,
        )
    }

    pub fn get_nearby(&self, pos: Vec2) -> Vec<usize> {
        let cell = self.get_cell(pos);
        let mut nearby = Vec::new();

        // Check 3x3 grid around the cell
        for dx in -1..=1 {
            for dy in -1..=1 {
                let check_cell = (cell.0 + dx, cell.1 + dy);
                if let Some(indices) = self.cells.get(&check_cell) {
                    nearby.extend(indices);
                }
            }
        }
        nearby
    }
}
