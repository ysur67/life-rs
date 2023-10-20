use super::cell::Cell;

pub struct Playfield {
    pub size: usize,
    cells: Vec<Vec<Cell>>,
}

impl Playfield {
    pub fn create(size: usize) -> Self {
        let cells: Vec<Vec<Cell>> = (1..size)
            .map(|_| (1..size).map(|_| Cell::create(None)).collect())
            .collect();
        return Playfield { size, cells };
    }
}
