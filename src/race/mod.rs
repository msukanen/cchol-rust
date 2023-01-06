pub mod human;
pub mod elf;
pub mod dwarf;
pub mod halfling;
pub mod monster;
pub mod hybrid;

use crate::{dice::DiceExt, society::culture::CultureType};
use crate::event::racial_event::RacialEventType;

use self::{
    elf::Elf,
    human::Human,
    dwarf::Dwarf,
    halfling::Halfling,
    hybrid::{
        halfelf::HalfElf,
        halforc::HalfOrc
    },
    monster::{
        beastman::BeastmanFactory,
        reptileman::ReptilemanFactory,
        orc::Orc
    }
};

/// Traits for all current and future races.
pub trait Race {
    /// Name of the race.
    fn name(&self) -> &'static str;
    /// Description of the race.
    fn description(&self) -> &'static str;
    /// Racial event type.
    fn event_type(&self) -> RacialEventType;
    /// Maximum culture type.
    fn max_culture(&self) -> CultureType { CultureType::ANY }
}

pub trait RaceF {
    fn new() -> Box<dyn Race>;
}

pub struct RaceFactory;

impl RaceF for RaceFactory {
    /// Generate a random `Race`.
    fn new() -> Box<dyn Race> {
        let r = 1.d(20);
        match r {
            x if x < 14 => Self::new_human(),
            _ => Self::new_nonhuman()
        }
    }
}

impl RaceFactory {
    /// Generate a `Human` `Race`.
    pub fn new_human() -> Box<dyn Race> { Human::new() }

    /// Generate a random nonhuman `Race`.
    pub fn new_nonhuman() -> Box<dyn Race> {
        let r = 1.d(20);
        match r {
            x if x <= 4 => Elf::new(),
            x if x <= 8 => Dwarf::new(),
            x if x <= 11 => Halfling::new(),
            x if x <= 15 => HalfElf::new(1.d(2)==1),
            16 => BeastmanFactory::new(),
            17 => ReptilemanFactory::new(),
            18 => Orc::new(),
            _ => HalfOrc::new(1.d(2)==1)
        }
    }
}
