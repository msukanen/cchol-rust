use super::Race;
use crate::{event::racial_event::RacialEventType, society::culture::CultureType};

pub struct Halfling;

impl Race for Halfling {
    fn event_type(&self) -> RacialEventType { RacialEventType::HALFLING }
    fn name(&self) -> &'static str {"halfling"}
    fn description(&self) -> &'static str { self.name() }
    fn max_culture(&self) -> CultureType { CultureType::CIVILIZED }
}

impl Halfling {
    pub fn new() -> Box<dyn Race> {Box::new(Halfling{})}
}
