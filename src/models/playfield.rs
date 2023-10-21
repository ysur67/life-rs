use std::collections::HashMap;

use super::{cell::Square, position::Position};

pub struct Playfield {
    pub size: usize,
    cells: Vec<Vec<Square>>,
}

impl Playfield {
    pub fn get_cells(&self) -> &Vec<Vec<Square>> {
        return &self.cells;
    }

    pub fn get_alive_count(&self) -> usize {
        return self
            .cells
            .iter()
            .flatten()
            .filter(|el| el.is_alive())
            .count();
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

    pub fn new(size: usize) -> Self {
        let cells: Vec<Vec<Square>> = (0..size)
            .map(|_| (0..size).map(|_| Square::new(None)).collect())
            .collect();
        return Playfield { size, cells };
    }

    pub fn apply_changes(&mut self, changes: &HashMap<Position, Square>) {
        for (key, value) in changes.iter() {
            self.cells[key.row][key.col] = value.clone();
        }
    }
}
