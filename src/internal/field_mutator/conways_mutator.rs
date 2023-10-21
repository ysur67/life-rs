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
                let new_square = Square::new(if !square.is_alive() {
                    Some(alive_count == 3)
                } else {
                    Some(alive_count == 3 || alive_count == 2)
                });
                changes.insert(Position::new(row_index, col_index), new_square);
            }
        }
        field.apply_changes(&changes)
    }
}

#[cfg(test)]
mod tests {
    use crate::internal::{
        field_manager::forloop_based_manager::ForLoopBasedPlayfieldManager,
        patterns::{build_beacon_pattern, build_blinker_pattern},
    };

    use super::*;

    #[test]
    fn test_mutate_without_alive_squares() {
        let size = 25;
        let manager = ForLoopBasedPlayfieldManager {};
        let mutator = ConwaysRulesPlayfieldMutator::new(&manager);
        let mut field = Playfield::new(size);
        mutator.mutate(&mut field);
        assert_eq!(field.get_alive_count(), 0);
    }

    #[test]
    fn test_mutate_on_blinker_pattern() {
        let size = 25;
        let manager = ForLoopBasedPlayfieldManager {};
        let mutator = ConwaysRulesPlayfieldMutator::new(&manager);
        let mut field = Playfield::new(size);
        let pattern = build_blinker_pattern();
        field.apply_changes(&pattern);
        mutator.mutate(&mut field);
        let blinker_all_gens_alive_count = 3;
        assert_eq!(field.get_alive_count(), blinker_all_gens_alive_count);
    }

    #[test]
    fn test_mutate_on_beacon_pattern() {
        let size = 25;
        let manager = ForLoopBasedPlayfieldManager {};
        let mutator = ConwaysRulesPlayfieldMutator::new(&manager);
        let mut field = Playfield::new(size);
        let pattern = build_beacon_pattern();
        field.apply_changes(&pattern);
        let beacon_first_gen_alive_count = 8;
        assert_eq!(field.get_alive_count(), beacon_first_gen_alive_count);
        mutator.mutate(&mut field);
        let beacon_second_gen_alive_count = 6;
        assert_eq!(field.get_alive_count(), beacon_second_gen_alive_count);
        let cells = field.get_cells().clone();
        let alive_positions = vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(1, 0),
            Position::new(2, 3),
            Position::new(3, 2),
            Position::new(3, 3),
        ];
        for pos in alive_positions {
            assert!(cells[pos.row][pos.col].is_alive());
        }
        let dead_positions = vec![Position::new(1, 1), Position::new(2, 2)];
        for pos in dead_positions {
            assert!(cells[pos.row][pos.col].is_alive() == false)
        }
    }
}
