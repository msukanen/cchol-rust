use super::Race;
use crate::event::racial_event::RacialEventType;

pub struct Elf;

impl Race for Elf {
    fn event_type(&self) -> RacialEventType { RacialEventType::ELF }
    fn name(&self) -> &'static str { "elf" }
    fn description(&self) -> &'static str { "elf" }
}

impl Elf {
    pub fn new() -> Self {
        Elf { }
    }
}
