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
        for i in [-1, 0, 1] {
            for j in [-1, 0, 1] {
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

    fn get_alive_cells_around_count(&self, row: usize, col: usize, field: &Playfield) -> usize {
        let cells = self.get_cells_around(row, col, field);
        return cells.iter().filter(|el| el.is_alive()).count();
    }
}
