use crate::race::{monster::MonsterRace, Race};
use crate::event::racial_event::RacialEventType;

pub struct Orc;

impl Race for Orc {
    fn name(&self) -> &'static str {"orc"}
    fn description(&self) -> &'static str { self.name()}
    fn event_type(&self) -> RacialEventType { RacialEventType::MONSTER }
}

impl MonsterRace for Orc {

}

impl Orc {
    pub fn new() -> Box<dyn Race> { Box::new(Orc{})}
}
