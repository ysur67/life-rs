use std::{thread, time::Duration};

use internal::{
    field_manager::forloop_based_manager::ForLoopBasedPlayfieldManager,
    field_mutator::{conways_mutator::ConwaysRulesPlayfieldMutator, mutator::PlayfieldMutator},
    patterns::build_beacon_pattern,
};

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
    let size = 25;
    let mut field = Playfield::new(size);
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
