use std::collections::HashMap;

use crate::models::{cell::Square, playfield::Playfield, position::Position};

pub trait PlayfieldMutator {
    fn mutate(&self, field: &Playfield) -> HashMap<Position, Square>;
}