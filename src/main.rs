use crate::{
    internal::field_printer::{
        console_printer::ConsolePlayfieldPrinter, printer::PlayfieldPrinter,
    },
    models::playfield::Playfield,
};

mod internal;
mod models;

fn main() {
    loop {
        let field = Playfield::create(100);
        println!("{}", field.size);
        let printer = ConsolePlayfieldPrinter {};
        printer.display(field);
    }
}
