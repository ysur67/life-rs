#[derive(Copy, Clone)]
pub struct Square {
    is_alive: bool,
}

impl Square {
    pub fn create(is_alive: Option<bool>) -> Self {
        return Square {
            is_alive: is_alive.unwrap_or(false),
        };
    }

    pub fn is_alive(&self) -> bool {
        return self.is_alive;
    }

    pub fn set_is_alive(&mut self, is_alive: bool) {
        self.is_alive = is_alive;
    }
}
