

use crate::models::{playfield::Playfield};

pub trait PlayfieldMutator {
    fn mutate(&self, field: &mut Playfield);
}
