use crate::race::{monster::MonsterRace, Race};
use crate::event::racial_event::RacialEventType;

pub struct Goatman;

impl Race for Goatman {
    fn name(&self) -> &'static str {"goatman"}
    fn description(&self) -> &'static str { self.name()}
    fn event_type(&self) -> RacialEventType { RacialEventType::MONSTER }
}

impl MonsterRace for Goatman {

}

impl Goatman {
    pub fn new() -> Box<dyn Race> { Box::new(Goatman{})}
}
