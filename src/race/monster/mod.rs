pub mod beastman;
pub mod reptileman;

use crate::event::racial_event::RacialEventType;

trait MonsterRace {
    fn event_type(&self) -> RacialEventType { RacialEventType::MONSTER }
}
