use dicebag::DiceExt;
use rpga_generic::{body_location::BodyLocation, skill::direction::Direction};

use crate::attribute::{Attrib, AttribType};

pub enum SeriousWound {
    Combined(Vec<SeriousWound>),
    SW1(Attrib),
    SW2(BodyLocation),
    SW3(Direction),
    SW4(i32),
    SW5 { ear: Direction, permanent: bool },
    SW6(Vec<Attrib>),
}

impl SeriousWound {
    pub fn random() -> Self {
        match 1.d20() {
            ..=1 => Self::SW1(Attrib::new_valued(AttribType::CHA, if 1.d2() == 1 {-1} else {1})),
            2 => Self::SW2(BodyLocation::random()),
            3 => Self::SW3(Direction::random_lr()),
            4 => Self::SW4(1.d4()),
            5 => Self::SW5 { ear: Direction::random_lr(), permanent: 1.d10() > 6 },
            6 => Self::SW6(vec![Attrib::new_valued(AttribType::APP, -1.d10()), Attrib::new_valued(AttribType::CHA, -1.d10())]),
        }
    }
}
