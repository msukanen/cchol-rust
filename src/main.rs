use cchol::{race::{RaceFactory, RaceF}, society::culture::culture::CultureFactory};

fn main() {
    let r = <RaceFactory as RaceF>::new();
    let c = CultureFactory::new(r.as_ref());
    println!("Hello, {} {}!", c.level(), r.name());
}
