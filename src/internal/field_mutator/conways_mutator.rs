use std::collections::HashMap;

use crate::{
    internal::field_manager::manager::PlayfieldManager,
    models::{cell::Square, playfield::Playfield, position::Position},
};

use super::mutator::PlayfieldMutator;

pub struct ConwaysRulesPlayfieldMutator<'a> {
    manager: &'a (dyn PlayfieldManager + 'a),
}

impl<'a> ConwaysRulesPlayfieldMutator<'a> {
    pub fn new(manager: &'a impl PlayfieldManager) -> Self {
        return ConwaysRulesPlayfieldMutator { manager };
    }
}

impl<'a> PlayfieldMutator for ConwaysRulesPlayfieldMutator<'a> {
    fn mutate(&self, field: &mut Playfield) {
        let mut changes: HashMap<Position, Square> = HashMap::new();
        for (row_index, row) in field.get_cells().iter().enumerate() {
            for (col_index, square) in row.iter().enumerate() {
                let alive_count = self
                    .manager
                    .get_alive_cells_around_count(row_index, col_index, field);
                let new_square = Square::create(if !square.is_alive() {
                    Some(alive_count == 3)
                } else {
                    Some(alive_count == 3 || alive_count == 2)
                });
                changes.insert(Position::create(row_index, col_index), new_square);
            }
        }
        field.apply_changes(&changes)
    }
}
