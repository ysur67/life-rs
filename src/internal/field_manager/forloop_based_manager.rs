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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_for_loop_based_manager_get_cells_around() {
        let field_size = 100;
        let field = Playfield::create(field_size + 1);
        let manager = ForLoopBasedPlayfieldManager {};
        let cells_around_for_corner_cell_size = 3;
        let left_top_corner = manager.get_cells_around(0, 0, &field);
        assert_eq!(left_top_corner.len(), cells_around_for_corner_cell_size);
        let right_top_corner = manager.get_cells_around(0, field_size, &field);
        assert_eq!(right_top_corner.len(), cells_around_for_corner_cell_size);
        let left_bottom_corner = manager.get_cells_around(field_size, 0, &field);
        assert_eq!(left_bottom_corner.len(), cells_around_for_corner_cell_size);
        let right_bottom_corner = manager.get_cells_around(field_size, field_size, &field);
        assert_eq!(right_bottom_corner.len(), cells_around_for_corner_cell_size);

        let any_cell = manager.get_cells_around(10, 10, &field);
        assert_eq!(any_cell.len(), 8);
    }
}
