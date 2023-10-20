use crate::models::playfield::Playfield;

mod models;

fn main() {
    println!("Hello, world!");
    let field = Playfield::create(100);
    println!("{}", field.size);
    let _cells_around = field.get_cells_around(0, 0);
    println!("jopa");
}
