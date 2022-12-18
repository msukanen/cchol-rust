pub mod centaur;
pub mod minotaur;
pub mod satyr;
pub mod faun;
pub mod goatman;

use crate::{race::{Race, RaceF}, dice::DiceExt};
use self::{centaur::Centaur, minotaur::Minotaur, faun::Faun, satyr::Satyr, goatman::Goatman};

pub trait BeastmanRace {

}

pub struct BeastmanFactory;

impl RaceF for BeastmanFactory {
    fn new() -> Box<dyn Race> {
        let r = 1.d(8);
        match r {
            x if x <= 3 => Centaur::new(),
            x if x <= 5 => Minotaur::new(),
            x if x <= 8 => Faun::new(),
            9 => Satyr::new(),
            _ => Goatman::new()
        }
    }
}
