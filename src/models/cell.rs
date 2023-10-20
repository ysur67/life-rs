#[derive(Copy, Clone)]
pub struct Square {
    pub is_alive: bool,
    pub row: usize,
    pub col: usize,
}

impl Square {
    pub fn create(is_alive: Option<bool>, row: usize, col: usize) -> Self {
        return Square {
            is_alive: is_alive.unwrap_or(false),
            row,
            col,
        };
    }
}
