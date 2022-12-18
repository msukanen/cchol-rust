use crate::{event::racial_event::RacialEventType, race::{monster::MonsterRace, Race}};

pub struct Minotaur;

impl Race for Minotaur {
    fn name(&self) -> &'static str {"minotaur"}
    fn description(&self) -> &'static str { self.name()}
    fn event_type(&self) -> crate::event::racial_event::RacialEventType {RacialEventType::MONSTER}
}

impl MonsterRace for Minotaur {

}

impl Minotaur {
    pub fn new() -> Box<dyn Race> { Box::new(Minotaur{})}
}
