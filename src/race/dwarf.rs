use super::Race;
use crate::{event::racial_event::RacialEventType, society::culture::CultureType};

pub struct Dwarf;

impl Race for Dwarf {
    fn event_type(&self) -> RacialEventType { RacialEventType::DWARF }
    fn name(&self) -> &'static str { "dwarf" }
    fn description(&self) -> &'static str { "dwarf" }
    fn max_culture(&self) -> CultureType { CultureType::CIVILIZED }
}

impl Dwarf {
    pub fn new() -> Box<dyn Race> {Box::new(Dwarf{})}
}
