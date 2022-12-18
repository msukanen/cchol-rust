pub mod human;
pub mod elf;
pub mod dwarf;
pub mod halfling;
pub mod monster;
pub mod hybrid;

use crate::dice::DiceExt;
use crate::event::racial_event::RacialEventType;

use self::{elf::Elf, human::Human, dwarf::Dwarf, halfling::Halfling, hybrid::{halfelf::HalfElf}};

/// Traits for all current and future races.
pub trait Race {
    /// Name of the race.
    fn name(&self) -> &'static str;
    /// Description of the race.
    fn description(&self) -> &'static str;
    /// Racial event type.
    fn event_type(&self) -> RacialEventType;
}

pub trait HybridRace {
    /// Raised by `Human`s?
    fn raised_by_humans(&self) -> bool;
}

pub struct RaceFactory;

impl RaceFactory {
    /// Generate a random `Race`.
    pub fn new() -> Box<dyn Race> {
        let r = 1.d(20);
        match r {
            x if x < 14 => Self::new_human(),
            _ => Self::new_nonhuman()
        }
    }

    /// Generate a `Human` `Race`.
    pub fn new_human() -> Box<dyn Race> { Box::new(Human::new()) }

    /// Generate a random nonhuman `Race`.
    pub fn new_nonhuman() -> Box<dyn Race> {
        let r = 1.d(20);
        match r {
            x if x <= 4 => Box::new(Elf::new()),
            x if x <= 8 => Box::new(Dwarf::new()),
            x if x <= 11 => Box::new(Halfling::new()),
            x if x <= 15 => Box::new(HalfElf::new(1.d(2)==1)),
            _ => Self::new_human()
        }
    }
}
