use crate::models::{cell::Square, playfield::Playfield};

pub struct PlayfieldManager {}

impl PlayfieldManager {
    pub fn get_cells_around(&self, row: usize, col: usize, field: &Playfield) -> Vec<Square> {
        if row > field.inner_size || col > field.inner_size {
            return vec![];
        }
        let inner_row = if row > 0 { row - 1 } else { row };
        let inner_col = if col > 0 { col - 1 } else { col };
        let mut lt: Option<Square> = None;
        let mut rt: Option<Square> = None;
        let mut mt: Option<Square> = None;
        let mut lb: Option<Square> = None;
        let mut rb: Option<Square> = None;
        let mut mb: Option<Square> = None;
        let mut lm: Option<Square> = None;
        let mut rm: Option<Square> = None;
        let cells = field.get_cells();
        let bottom_row = cells.get(inner_row + 1);
        if inner_row != 0 {
            let top_row = cells.get(inner_row - 1);
            if top_row.is_some() {
                let index = inner_row - 1;
                if inner_col != 0 {
                    let left_col = cells[index].get(inner_col - 1);
                    if left_col.is_some() {
                        lt = Some(cells[index][inner_col - 1]);
                    }
                }
                let right_col = cells[index].get(inner_col + 1);
                let top_col = cells[index].get(inner_col);
                if right_col.is_some() {
                    rt = Some(cells[index][inner_col + 1]);
                }
                if top_col.is_some() {
                    mt = Some(cells[index][inner_col]);
                }
            }
        }
        if bottom_row.is_some() {
            let index = inner_row + 1;
            if inner_col != 0 {
                let left_col = cells[index].get(inner_col - 1);
                if left_col.is_some() {
                    lb = Some(cells[index][inner_col - 1]);
                }
            }
            let right_col = cells[index].get(inner_col + 1);
            let bottom_col = cells[index].get(inner_col);
            if right_col.is_some() {
                rb = Some(cells[index][inner_col + 1]);
            }
            if bottom_col.is_some() {
                mb = Some(cells[index][inner_col]);
            }
        }
        if inner_col != 0 {
            let left_col = cells[inner_row].get(inner_col - 1);
            if left_col.is_some() {
                lm = Some(cells[inner_row][inner_col - 1]);
            }
        }
        let right_col = cells[inner_row].get(inner_col + 1);
        if right_col.is_some() {
            rm = Some(cells[inner_row][inner_col + 1]);
        }
        let raw_result = vec![lt, rt, rb, lb, lm, rm, mt, mb];
        return raw_result
            .iter()
            .filter(|el| el.is_some())
            .map(|el| el.unwrap())
            .collect();
    }
}
