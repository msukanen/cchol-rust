use crate::{race::{monster::MonsterRace, Race}, event::racial_event::RacialEventType, society::culture::CultureType};

pub struct Centaur;

impl Race for Centaur {
    fn name(&self) -> &'static str {"centaur"}
    fn description(&self) -> &'static str { self.name()}
    fn event_type(&self) -> RacialEventType { RacialEventType::MONSTER }
    fn max_culture(&self) -> CultureType { CultureType::CIVILIZED }
}

impl MonsterRace for Centaur {
    
}

impl Centaur {
    pub fn new() -> Box<dyn Race> { Box::new(Centaur{})}
}
