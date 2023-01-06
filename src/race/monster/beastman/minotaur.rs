use crate::{event::racial_event::RacialEventType, race::{monster::MonsterRace, Race}, society::culture::CultureType};

pub struct Minotaur;

impl Race for Minotaur {
    fn name(&self) -> &'static str {"minotaur"}
    fn description(&self) -> &'static str { self.name() }
    fn event_type(&self) -> crate::event::racial_event::RacialEventType { RacialEventType::MONSTER }
    fn max_culture(&self) -> CultureType { CultureType::BARBARIAN }
}

impl MonsterRace for Minotaur {

}

impl Minotaur {
    pub fn new() -> Box<dyn Race> { Box::new(Minotaur{})}
}
