use crate::models::{cell::Square, playfield::Playfield};

use super::manager::PlayfieldManager;

pub struct ForLoopBasedPlayfieldManager {}

impl PlayfieldManager for ForLoopBasedPlayfieldManager {
    fn get_cells_around(
        &self,
        row: usize,
        col: usize,
        field: &Playfield,
    ) -> Vec<crate::models::cell::Square> {
        let mut result: Vec<Square> = vec![];
        let mut _row = row as isize;
        let mut _col = col as isize;
        if _row != 0 {
            _row = _row - 1;
        }
        if _col != 0 {
            _col = _col - 1;
        }
        for i in -1..=1isize {
            for j in -1..=1isize {
                if i == 0 && j == 0 {
                    continue;
                }
                let square = field.get_square(i + _row, j + _col);
                if square.is_some() {
                    result.push(square.unwrap());
                }
            }
        }
        return result;
    }
}
