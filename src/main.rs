use cchol::race::RaceFactory;

fn main() {
    let r = RaceFactory::new();
    println!("Hello, {} world!", r.name());
}
