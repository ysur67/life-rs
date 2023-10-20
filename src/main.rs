use std::{collections::HashMap, thread, time::Duration};

use internal::{
    field_manager::PlayfieldManager,
    field_mutator::{
        conways_mutator::ConwaysRulesPlayfieldMutator,
        mutator::{self, PlayfieldMutator},
    },
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
    let manager = PlayfieldManager {};
    let mutator = ConwaysRulesPlayfieldMutator::new(manager);
    let size = 20;
    let mut field = Playfield::create(size);
    let mut initial_changes = HashMap::new();
    for index in 0..size {
        initial_changes.insert(
            Position::create(index, index),
            Square::create(Some(true), index, index),
        );
    }
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
        thread::sleep(Duration::from_millis(300));
    }
}
