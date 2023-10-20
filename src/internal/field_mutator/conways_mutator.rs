use std::collections::HashMap;

use crate::{
    internal::field_manager::PlayfieldManager,
    models::{cell::Square, playfield::Playfield, position::Position},
};

use super::mutator::PlayfieldMutator;

pub struct ConwaysRulesPlayfieldMutator {}

impl PlayfieldMutator for ConwaysRulesPlayfieldMutator {
    fn mutate(&self, field: &Playfield) -> HashMap<Position, Square> {
        let mut result: HashMap<Position, Square> = HashMap::new();
        let manager = PlayfieldManager {};
        for (row_index, row) in field.get_cells().iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                let around_cells = manager.get_cells_around(row_index, col_index, &field);
                if cell.is_alive == false {
                    result.insert(
                        Position::create(row_index, col_index),
                        Square::create(Some(around_cells.len() == 3), row_index, col_index),
                    );
                } else {
                    result.insert(
                        Position::create(row_index, col_index),
                        Square::create(
                            Some(around_cells.len() == 3 || around_cells.len() == 2),
                            row_index,
                            col_index,
                        ),
                    );
                }
            }
        }
        return result;
    }
}

impl ConwaysRulesPlayfieldMutator {}
