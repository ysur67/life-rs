use crate::models::playfield::Playfield;

mod models;

fn main() {
    println!("Hello, world!");
    let field = Playfield::create(100);
    print!("{}", field.size);
}
