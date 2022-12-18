pub struct Reptileman;

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

impl ReptilemanRace for Reptileman {}
impl Race for Reptileman {
    fn name(&self) -> &'static str {"reptileman"}
    fn description(&self) -> &'static str {self.name()}
    fn event_type(&self) -> crate::event::racial_event::RacialEventType {RacialEventType::MONSTER}
}

impl MonsterRace for Reptileman {
    
}

impl Reptileman {
    pub fn new() -> Box<dyn Race> {Box::new(Reptileman{})}
}
