use std::collections::HashMap;

use crate::models::{cell::Cell, playfield::Playfield, position::Position};

use super::mutator::PlayfieldMutator;

pub struct ConwaysRulesPlayfieldMutator {}

impl PlayfieldMutator for ConwaysRulesPlayfieldMutator {
    fn mutate(&self, field: &Playfield) -> HashMap<Position, Cell> {
        let mut result: HashMap<Position, Cell> = HashMap::new();
        for (row_index, row) in field.get_cells().iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                let around_cells = field.get_cells_around(row_index, col_index);
                if !cell.is_alive {
                    if around_cells.len() == 3 {
                        result.insert(
                            Position::create(row_index, col_index),
                            Cell::create(Some(false)),
                        );
                    }
                } else {
                    result.insert(
                        Position::create(row_index, col_index),
                        Cell::create(Some(around_cells.len() == 3 || around_cells.len() == 2)),
                    );
                }
            }
        }
        return result;
    }
}
