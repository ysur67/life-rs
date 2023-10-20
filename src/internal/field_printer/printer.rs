use crate::models::playfield::Playfield;

pub trait PlayfieldPrinter {
    fn display(&self, field: Playfield);
}
