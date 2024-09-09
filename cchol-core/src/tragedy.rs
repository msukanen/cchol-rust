use dicebag::DiceExt;
use rpga_generic::gender::Gender;
use rpga_traits::Modifiered;
use small_ones::{ChildDies, PossessionVanished, SevereInjury};

use crate::{culture::Culture, misc::seriouswound::SeriousWound, social_level::status::Status};

pub mod small_ones;

pub enum Tragedy {
    TN2 { wound: SeriousWound, relative: Relative },
    T0 { crime: Crime, imprisoned: Imprisoned },
    T1(ChildDies),
    T2(DeathOfParent/*or Guardian*/),
    T3(Option<Other>),
    T4(DeathOfParent),
    T5 { utter_destruction: bool, death_toll: Vec<(FriendOrFamily, TragicDeathReason)> },
    T6 { other_person: Other, death_situation: DeathSituation },
    T7(DeathOfParent),
    T8 { death_toll: Vec<(FriendOrFamily, TragicDeathReason)> },
    T9(PossessionVanished),
    T10 { outlawed_parent: Vec<Gender>, crime: Crime, hiding_in_culture: Option<Culture>},
    T11(Enslaved),
    T12(SevereInjury),
}

impl Tragedy {
    pub fn random(status: &Status) -> Self {
        fn choice(solmod: i32) -> Tragedy {
            match 1.d20() + solmod {
                ..=-2 => Tragedy::TN2 { wound: (), relative: ()},
                -1 => choice(0),
                0 => Tragedy::T0 { crime: (), imprisoned: ()},
                1 => Tragedy::T1(ChildDies::random()),
                2 => Tragedy::T2(DeathOfParent::random()),
                3 => Tragedy::T3(if 1.d6() > 4 {Some(Other::random8)} else {None}),
                4 => Tragedy::T4(DeathOfParent::random()),
                5 => Tragedy::T5 { utter_destruction: 1.d6() == 6, death_toll: () },
                6 => Tragedy::T6 { other_person: Other::random(), death_situation: DeathSituation::random() },
                7 => Tragedy::T7(DeathOfParent::random()),
                8 => Tragedy::T8 { death_toll: () },
                9 => Tragedy::T9(PossessionVanished::random()),
                10 => Tragedy::T10 { outlawed_parent: (), crime: Crime::random(), hiding_in_culture: if 1.d6() > 4 {Some(Culture::random())} else {None}},
            }
        }

        choice(status.modifier())
    }
}
