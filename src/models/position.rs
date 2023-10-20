#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn create(row: usize, col: usize) -> Self {
        return Position { row, col };
    }
}
