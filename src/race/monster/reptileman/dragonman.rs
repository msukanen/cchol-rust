pub struct Dragonman;

use crate::{
    event::racial_event::RacialEventType,
    race::{
        Race,
        monster::{
            reptileman::ReptilemanRace,
            MonsterRace
        }
    }
};

impl ReptilemanRace for Dragonman {}
impl Race for Dragonman {
    fn name(&self) -> &'static str {"dragonman"}
    fn description(&self) -> &'static str {self.name()}
    fn event_type(&self) -> crate::event::racial_event::RacialEventType {RacialEventType::MONSTER}
}

impl MonsterRace for Dragonman {
    
}

impl Dragonman {
    pub fn new() -> Box<dyn Race> {Box::new(Dragonman{})}
}
