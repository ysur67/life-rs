use std::collections::HashMap;

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

    pub fn get_square(&self, row: isize, col: isize) -> Option<Square> {
        let _row = self.cells.get(row as usize);
        if _row.is_none() {
            return None;
        }
        let _col = _row.unwrap().get(col as usize);
        if _col.is_none() {
            return None;
        }
        return _col.map(|el| el.clone());
    }

    pub fn create(size: usize) -> Self {
        let inner_size = size;
        let cells: Vec<Vec<Square>> = (0..inner_size)
            .map(|_| (0..inner_size).map(|_| Square::create(None)).collect())
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
