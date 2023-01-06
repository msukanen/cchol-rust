pub struct Serpentman;

use crate::{
    event::racial_event::RacialEventType,
    race::{
        Race,
        monster::{
            reptileman::ReptilemanRace,
            MonsterRace
        }
    },
};

impl ReptilemanRace for Serpentman {}
impl Race for Serpentman {
    fn name(&self) -> &'static str {"serpentman"}
    fn description(&self) -> &'static str { self.name() }
    fn event_type(&self) -> RacialEventType { RacialEventType::MONSTER }
}

impl MonsterRace for Serpentman {
    
}

impl Serpentman {
    pub fn new() -> Box<dyn Race> {Box::new(Serpentman{})}
}
