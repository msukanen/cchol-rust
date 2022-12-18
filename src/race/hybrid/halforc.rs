use crate::race::{Race, HybridRace};
use crate::event::racial_event::RacialEventType;

pub struct HalfOrc {
    raised_by_human: bool
}

impl Race for HalfOrc {
    fn event_type(&self) -> RacialEventType {
        if self.raised_by_human { RacialEventType::HUMAN   }
        else                    { RacialEventType::MONSTER }
     }
    fn name(&self) -> &'static str { "human" }
    fn description(&self) -> &'static str { "human" }
}

impl HalfOrc {
    pub fn new(raised_by_human: bool) -> Self {
        HalfOrc { raised_by_human }
    }
}

impl HybridRace for HalfOrc {
    fn raised_by_humans(&self) -> bool { self.raised_by_human }
}
