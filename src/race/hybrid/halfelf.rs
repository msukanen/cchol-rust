use crate::race::Race;
use crate::event::racial_event::RacialEventType;

use super::HybridRace;

pub struct HalfElf {
    raised_by_human: bool
}

impl Race for HalfElf {
    fn event_type(&self) -> RacialEventType {
        if self.raised_by_human { RacialEventType::HUMAN }
        else                    { RacialEventType::ELF   }
     }
    fn name(&self) -> &'static str { "half-elf" }
    fn description(&self) -> &'static str { self.name() }
}

impl HalfElf {
    pub fn new(raised_by_human: bool) -> Box<dyn Race> {
        Box::new(HalfElf { raised_by_human })
    }
}

impl HybridRace for HalfElf {
    fn raised_by_humans(&self) -> bool { self.raised_by_human }
}
