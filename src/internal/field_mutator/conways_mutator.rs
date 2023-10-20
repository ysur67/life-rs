use crate::{internal::field_manager::PlayfieldManager, models::playfield::Playfield};

use super::mutator::PlayfieldMutator;

pub struct ConwaysRulesPlayfieldMutator {
    manager: PlayfieldManager,
}

impl ConwaysRulesPlayfieldMutator {
    pub fn new(manager: PlayfieldManager) -> Self {
        return ConwaysRulesPlayfieldMutator { manager };
    }
}

impl PlayfieldMutator for ConwaysRulesPlayfieldMutator {
    fn mutate(&self, field: &mut Playfield) {
        let mut cells = field.get_cells().clone();
        for (row_index, row) in cells.iter_mut().enumerate() {
            for (col_index, square) in row.iter_mut().enumerate() {
                let around_cells = self.manager.get_cells_around(row_index, col_index, &field);
                let alive_count = around_cells.iter().filter(|el| el.is_alive()).count();
                square.set_is_alive(if !square.is_alive() {
                    alive_count == 3
                } else {
                    alive_count == 3 || alive_count == 2
                });
            }
        }
        field.update_field(cells);
    }
}
