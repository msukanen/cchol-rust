use super::Race;
use crate::event::racial_event::RacialEventType;

pub struct Elf;

impl Race for Elf {
    fn event_type(&self) -> RacialEventType { RacialEventType::ELF }
    fn name(&self) -> &'static str { "elf" }
    fn description(&self) -> &'static str { self.name() }
}

impl Elf {
    pub fn new() -> Box<dyn Race> {Box::new(Elf{})}
}
