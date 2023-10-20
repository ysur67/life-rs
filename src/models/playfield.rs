use std::{cell::Cell, collections::HashMap};

use super::{cell::Square, position::Position};

pub struct Playfield {
    pub size: usize,
    pub inner_size: usize,
    cells: Vec<Vec<Square>>,
}

impl Playfield {
    pub fn get_cells(&self) -> &Vec<Vec<Square>> {
        return &self.cells;
    }

    pub fn get_alive_count(&self) -> usize {
        return self.cells.len();
    }

    pub fn create(size: usize) -> Self {
        let inner_size = size;
        let cells: Vec<Vec<Square>> = (0..inner_size)
            .map(|row| {
                (0..inner_size)
                    .map(|col| Square::create(None, row, col))
                    .collect()
            })
            .collect();
        return Playfield {
            size,
            cells,
            inner_size,
        };
    }

    pub fn apply_changes(&mut self, changes: &HashMap<Position, Square>) {
        for (key, value) in changes.iter() {
            self.cells[key.row][key.col] = value.clone();
        }
    }

    pub fn update_field(&mut self, changes: Vec<Vec<Square>>) {
        self.cells = changes;
    }
}
