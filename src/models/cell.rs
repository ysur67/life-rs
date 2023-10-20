#[derive(Copy, Clone)]
pub struct Cell {
    pub is_alive: bool,
}

impl Cell {
    pub fn create(is_alive: Option<bool>) -> Self {
        return Cell {
            is_alive: is_alive.unwrap_or(false),
        };
    }
}
