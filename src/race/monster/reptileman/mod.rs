use crate::{race::{RaceF, Race}, dice::DiceExt};

use self::{dragonman::Dragonman, serpentman::Serpentman, reptileman::Reptileman};

pub mod reptileman;
pub mod dragonman;
pub mod serpentman;

pub trait ReptilemanRace {}

pub struct ReptilemanFactory;

impl RaceF for ReptilemanFactory {
    fn new() -> Box<dyn Race> {
        let r = 1.d(6);
        match r {
            6 => Dragonman::new(),
            4|5 => Serpentman::new(),
            _ => Reptileman::new()
        }
    }
}
