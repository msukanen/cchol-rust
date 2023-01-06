use crate::race::{monster::MonsterRace, Race};
use crate::event::racial_event::RacialEventType;
use crate::society::culture::CultureType;

pub struct Satyr;

impl Race for Satyr {
    fn name(&self) -> &'static str {"satyr"}
    fn description(&self) -> &'static str { self.name()}
    fn event_type(&self) -> RacialEventType { RacialEventType::MONSTER }
    fn max_culture(&self) -> CultureType { CultureType::BARBARIAN }
}

impl MonsterRace for Satyr {

}

impl Satyr {
    pub fn new() -> Box<dyn Race> { Box::new(Satyr{})}
}
