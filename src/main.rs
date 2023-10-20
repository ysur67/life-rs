use std::{collections::HashMap, thread, time::Duration};

use internal::{
    field_manager::PlayfieldManager, field_mutator::conways_mutator::ConwaysRulesPlayfieldMutator,
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
    let _mutator = ConwaysRulesPlayfieldMutator {};
    let manager = PlayfieldManager {};
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
        let mut cells = field.get_cells().clone();
        for (row_index, row) in cells.iter_mut().enumerate() {
            for (col_index, square) in row.iter_mut().enumerate() {
                let around_cells = manager.get_cells_around(row_index, col_index, &field);
                let is_alive: bool;
                let alive_count = around_cells.iter().filter(|el| el.is_alive).count();
                if square.is_alive == false {
                    is_alive = alive_count == 3;
                } else {
                    is_alive = alive_count == 3 || alive_count == 2;
                }
                square.is_alive = is_alive;
            }
        }
        field.update_whole(cells);
        let alive_cells = field.get_alive_count();
        if alive_cells == 0 {
            println!("Game Over");
            break;
        }
        thread::sleep(Duration::from_millis(300));
    }
}
