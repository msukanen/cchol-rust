use crate::race::{Race, HybridRace};
use crate::event::racial_event::RacialEventType;

pub struct HalfElf {
    raised_by_human: bool
}

impl Race for HalfElf {
    fn event_type(&self) -> RacialEventType {
        if self.raised_by_human { RacialEventType::HUMAN }
        else                    { RacialEventType::ELF   }
     }
    fn name(&self) -> &'static str { "human" }
    fn description(&self) -> &'static str { "human" }
}

impl HalfElf {
    pub fn new(raised_by_human: bool) -> Self {
        HalfElf { raised_by_human }
    }
}

impl HybridRace for HalfElf {
    fn raised_by_humans(&self) -> bool { self.raised_by_human }
}
