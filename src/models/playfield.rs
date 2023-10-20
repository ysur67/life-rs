use std::collections::HashMap;

use super::{cell::Cell, position::Position};

pub struct Playfield {
    pub size: usize,
    inner_size: usize,
    cells: Vec<Vec<Cell>>,
}

impl Playfield {
    pub fn get_cells(&self) -> &Vec<Vec<Cell>> {
        return &self.cells;
    }

    pub fn create(size: usize) -> Self {
        let inner_size = size + 1;
        let cells: Vec<Vec<Cell>> = (1..inner_size)
            .map(|_| (1..inner_size).map(|_| Cell::create(None)).collect())
            .collect();
        return Playfield {
            size,
            cells,
            inner_size,
        };
    }

    pub fn apply_changes(&mut self, changes: &HashMap<Position, Cell>) {
        for (key, value) in changes.iter() {
            self.cells[key.row][key.col] = *value;
        }
    }

    pub fn get_cells_around(&self, row: usize, col: usize) -> Vec<Cell> {
        if row > self.inner_size || col > self.inner_size {
            return vec![];
        }
        let inner_row = if row > 0 { row - 1 } else { row };
        let inner_col = if col > 0 { col - 1 } else { col };
        let mut lt: Option<Cell> = None;
        let mut rt: Option<Cell> = None;
        let mut mt: Option<Cell> = None;
        let mut lb: Option<Cell> = None;
        let mut rb: Option<Cell> = None;
        let mut mb: Option<Cell> = None;
        let mut lm: Option<Cell> = None;
        let mut rm: Option<Cell> = None;
        let bottom_row = self.cells.get(inner_row + 1);
        if inner_row != 0 {
            let top_row = self.cells.get(inner_row - 1);
            if top_row.is_some() {
                let index = inner_row - 1;
                if inner_col != 0 {
                    let left_col = self.cells[index].get(inner_col - 1);
                    if left_col.is_some() {
                        lt = Some(self.cells[index][inner_col - 1]);
                    }
                }
                let right_col = self.cells[index].get(inner_col + 1);
                let top_col = self.cells[index].get(inner_col);
                if right_col.is_some() {
                    rt = Some(self.cells[index][inner_col + 1]);
                }
                if top_col.is_some() {
                    mt = Some(self.cells[index][inner_col]);
                }
            }
        }
        if bottom_row.is_some() {
            let index = inner_row + 1;
            if inner_col != 0 {
                let left_col = self.cells[index].get(inner_col - 1);
                if left_col.is_some() {
                    lb = Some(self.cells[index][inner_col - 1]);
                }
            }
            let right_col = self.cells[index].get(inner_col + 1);
            let bottom_col = self.cells[index].get(inner_col);
            if right_col.is_some() {
                rb = Some(self.cells[index][inner_col + 1]);
            }
            if bottom_col.is_some() {
                mb = Some(self.cells[index][inner_col]);
            }
        }
        if inner_col != 0 {
            let left_col = self.cells[inner_row].get(inner_col - 1);
            if left_col.is_some() {
                lm = Some(self.cells[inner_row][inner_col - 1]);
            }
        }
        let right_col = self.cells[inner_row].get(inner_col + 1);
        if right_col.is_some() {
            rm = Some(self.cells[inner_row][inner_col + 1]);
        }
        let raw_result = vec![lt, rt, rb, lb, lm, rm, mt, mb];
        return raw_result
            .iter()
            .filter(|el| el.is_some())
            .map(|el| el.unwrap())
            .collect();
    }
}
