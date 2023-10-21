#[derive(Copy, Clone)]
pub struct Square {
    is_alive: bool,
}

impl Square {
    pub fn new(is_alive: Option<bool>) -> Self {
        return Square {
            is_alive: is_alive.unwrap_or(false),
        };
    }

    pub fn is_alive(&self) -> bool {
        return self.is_alive;
    }
}
