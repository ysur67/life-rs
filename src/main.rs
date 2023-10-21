use std::{collections::HashMap, thread, time::Duration};

use internal::{
    field_manager::forloop_based_manager::ForLoopBasedPlayfieldManager,
    field_mutator::{conways_mutator::ConwaysRulesPlayfieldMutator, mutator::PlayfieldMutator},
};
use models::{cell::Square, position::Position};

use crate::{
    internal::field_printer::{
        console_printer::ConsolePlayfieldPrinter, printer::PlayfieldPrinter,
    },
    models::playfield::Playfield,
};

mod internal;
mod models;

fn main() {
    let manager = ForLoopBasedPlayfieldManager {};
    let mutator = ConwaysRulesPlayfieldMutator::new(&manager);
    let size = 4;
    let mut field = Playfield::create(size);
    let initial_changes = build_beacon_pattern();
    field.apply_changes(&initial_changes);
    loop {
        let printer = ConsolePlayfieldPrinter {};
        printer.display(&field);
        mutator.mutate(&mut field);
        let alive_cells = field.get_alive_count();
        if alive_cells == 0 {
            println!("Game Over");
            break;
        }
        thread::sleep(Duration::from_millis(250));
    }
}

fn build_beacon_pattern() -> HashMap<Position, Square> {
    let mut result = HashMap::new();
    result.insert(Position::create(0, 0), Square::create(Some(true)));
    result.insert(Position::create(0, 1), Square::create(Some(true)));
    result.insert(Position::create(1, 0), Square::create(Some(true)));
    result.insert(Position::create(1, 1), Square::create(Some(true)));
    result.insert(Position::create(2, 2), Square::create(Some(true)));
    result.insert(Position::create(2, 3), Square::create(Some(true)));
    result.insert(Position::create(3, 2), Square::create(Some(true)));
    result.insert(Position::create(3, 3), Square::create(Some(true)));
    return result;
}
