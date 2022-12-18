use super::Event;

pub enum RacialEventType {
    HUMAN, ELF, DWARF, HALFLING, MONSTER,
}

trait RacialEvent {

}

impl<T> RacialEvent for T where T: Event
{
    
}
