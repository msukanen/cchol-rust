use cchol::race::{RaceFactory, RaceF};

fn main() {
    let r = <RaceFactory as RaceF>::new();
    println!("Hello, {} world!", r.name());
}
