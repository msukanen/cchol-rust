use cchol::{race::{RaceFactory, RaceF}, society::{culture::culture::CultureFactory, status::status::StatusFactory}};

fn main() {
    let r = <RaceFactory as RaceF>::new();
    let c = CultureFactory::new(r.as_ref());
    let s = StatusFactory::new(c.as_ref());
    println!("Hello, {} {} of {}!", c.level(), r.name(), "s.level()");
}
