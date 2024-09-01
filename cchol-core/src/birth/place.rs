use dicebag::DiceExt;

use crate::{culture::Culture, deities_t864::Deity};

pub enum PlaceOfBirth {
    P1,
    P2,
    P3,
    P4,
    P5(Box<PlaceOfBirth>),
    P6,
    P7,
    P8,
    Exotic(ExoticPlaceOfBirth),
}

pub enum ExoticPlaceOfBirth {
    Combined(Box<ExoticPlaceOfBirth>, Box<ExoticPlaceOfBirth>),
    P1 { deity: Deity },
    P2 { among_camp_followers: bool },
    P3,
    P4,
    P5,
    P6,
    P7,
    P8,
    P9,
    P10,
    P11,
    P12,
    P13 { deity: Deity },
    P14,
    P15,
    P16,
    P17,
    P18,
}

impl PlaceOfBirth {
    /**
     Generate random birth place.

     **Params**
     * `culture` - one or other [Culture] that might affect some result(s).
     * `legit_mod` - ***LegitMod***.
     
     **Returns** a random birth place.
     */
    pub fn random(culture: &Culture, legit_mod: i32) -> Self {
        fn choice(culture: &Culture, legit_mod: i32) -> PlaceOfBirth {
            match 1.d20() + legit_mod {
                ..=6 => PlaceOfBirth::P1,
                ..=9 => PlaceOfBirth::P2,
                10 => PlaceOfBirth::P3,
                11 => PlaceOfBirth::P4,
                ..=13 => PlaceOfBirth::P5(Box::new(choice(culture, legit_mod))),
                14 => PlaceOfBirth::P6,
                15 => PlaceOfBirth::P7,
                16 => PlaceOfBirth::P8,
                _  => PlaceOfBirth::Exotic(exotic_choice(culture))
            }
        }

        fn exotic_choice(culture: &Culture) -> ExoticPlaceOfBirth {
            match 1.d20() {
                ..=2 => ExoticPlaceOfBirth::Combined(Box::new(exotic_choice(culture)), Box::new(exotic_choice(culture))),
                3 => ExoticPlaceOfBirth::P1 { deity: Deity::random(culture, false)},
                4 => ExoticPlaceOfBirth::P2 { among_camp_followers: 1.d6() < 6 },
                5 => ExoticPlaceOfBirth::P3,
                6 => ExoticPlaceOfBirth::P4,
                7 => ExoticPlaceOfBirth::P5,
                8 => ExoticPlaceOfBirth::P6,
                9 => ExoticPlaceOfBirth::P7,
                10 => ExoticPlaceOfBirth::P8,
                11 => ExoticPlaceOfBirth::P9,
                12 => ExoticPlaceOfBirth::P10,
                13 => ExoticPlaceOfBirth::P11,
                14 => ExoticPlaceOfBirth::P12,
                15 => ExoticPlaceOfBirth::P13 { deity: Deity::random(culture, true)},
                16 => ExoticPlaceOfBirth::P14,
                17 => ExoticPlaceOfBirth::P15,
                18 => ExoticPlaceOfBirth::P16,
                19 => ExoticPlaceOfBirth::P17,
                _  => ExoticPlaceOfBirth::P18,
            }
        }

        choice(culture, legit_mod)
    }
}
