use internal::field_mutator::conways_mutator::ConwaysRulesPlayfieldMutator;

use crate::{
    internal::{
        field_mutator::mutator::PlayfieldMutator,
        field_printer::{console_printer::ConsolePlayfieldPrinter, printer::PlayfieldPrinter},
    },
    models::playfield::Playfield,
};

mod internal;
mod models;

fn main() {
    let mutator = ConwaysRulesPlayfieldMutator {};
    loop {
        let mut field = Playfield::create(100);
        println!("{}", field.size);
        let printer = ConsolePlayfieldPrinter {};
        printer.display(&field);
        let changes = mutator.mutate(&field);
        field.apply_changes(&changes);
    }
}
