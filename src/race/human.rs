use super::Race;
use crate::event::racial_event::RacialEventType;

pub struct Human;

impl Race for Human {
    fn event_type(&self) -> RacialEventType { RacialEventType::HUMAN }
    fn name(&self) -> &'static str { "human" }
    fn description(&self) -> &'static str { "human" }
}

impl Human {
    pub fn new() -> Self {
        Human { }
    }
}
