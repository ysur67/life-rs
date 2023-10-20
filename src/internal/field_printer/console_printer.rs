use crate::models::playfield::Playfield;

use super::printer::PlayfieldPrinter;

pub struct ConsolePlayfieldPrinter {}

impl PlayfieldPrinter for ConsolePlayfieldPrinter {
    fn display(&self, field: &Playfield) {
        print!("{}[2J", 27 as char);
        for row in field.get_cells() {
            let mut row_repr = "".to_string();
            for cell in row {
                let cell_repr = if cell.is_alive { "#" } else { "-" };
                row_repr.push_str(cell_repr);
            }
            println!("{}", row_repr);
        }
    }
}
