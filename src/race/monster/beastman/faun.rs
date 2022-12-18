use crate::race::{monster::MonsterRace, Race};
use crate::event::racial_event::RacialEventType;

pub struct Faun;

impl Race for Faun {
    fn name(&self) -> &'static str {"faun"}
    fn description(&self) -> &'static str { self.name()}
    fn event_type(&self) -> RacialEventType { RacialEventType::MONSTER }
}

impl MonsterRace for Faun {}

impl Faun {
    pub fn new() -> Box<dyn Race> { Box::new(Faun{})}
}
