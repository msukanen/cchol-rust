use crate::race::Race;
use crate::event::racial_event::RacialEventType;

use super::HybridRace;

pub struct HalfOrc {
    raised_by_human: bool
}

impl Race for HalfOrc {
    fn event_type(&self) -> RacialEventType {
        if self.raised_by_human { RacialEventType::HUMAN   }
        else                    { RacialEventType::MONSTER }
     }
    fn name(&self) -> &'static str { "half-orc" }
    fn description(&self) -> &'static str { self.name() }
}

impl HalfOrc {
    pub fn new(raised_by_human: bool) -> Box<dyn Race> {Box::new(HalfOrc { raised_by_human })}
}

impl HybridRace for HalfOrc {
    fn raised_by_humans(&self) -> bool { self.raised_by_human }
}
