use crate::race::{monster::MonsterRace, Race};
use crate::event::racial_event::RacialEventType;
use crate::society::culture::CultureType;

pub struct Faun;

impl Race for Faun {
    fn name(&self) -> &'static str {"faun"}
    fn description(&self) -> &'static str { self.name()}
    fn event_type(&self) -> RacialEventType { RacialEventType::MONSTER }
    fn max_culture(&self) -> CultureType { CultureType::BARBARIAN }
}

impl MonsterRace for Faun {}

impl Faun {
    pub fn new() -> Box<dyn Race> { Box::new(Faun{})}
}
