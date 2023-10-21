use crate::models::{cell::Square, playfield::Playfield};

pub trait PlayfieldManager {
    fn get_cells_around(&self, row: usize, col: usize, field: &Playfield) -> Vec<Square>;
    fn get_alive_cells_around_count(&self, row: usize, col: usize, field: &Playfield) -> usize;
}
