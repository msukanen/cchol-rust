use super::Race;
use crate::event::racial_event::RacialEventType;

pub struct Halfling;

impl Race for Halfling {
    fn event_type(&self) -> RacialEventType { RacialEventType::HALFLING }
    fn name(&self) -> &'static str { "halfling" }
    fn description(&self) -> &'static str { "halfling" }
}

impl Halfling {
    pub fn new() -> Self {
        Halfling { }
    }
}
