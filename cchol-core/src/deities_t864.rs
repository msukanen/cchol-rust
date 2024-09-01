use dicebag::DiceExt;
use rpga_generic::gender::Gender;
use rpga_traits::Modifiered;

use crate::culture::Culture;

/// Various deities.
pub enum Deity {
    AncestorWorship { evil: bool },
    BeastGods { evil: bool },
    HuntingGod { evil: bool, gender: Gender },
    Trickster { evil: bool, gender: Gender },
    EarthGoddess { decadent: bool },
    AgriculturalGoddess { evil: bool, decadent: bool, disguised: bool },
    RulingDeity { evil: bool, gender: Gender, decadent: bool, disguised: bool },
    WaterGod { evil: bool, gender: Gender, decadent: bool, disguised: bool },
    FireGod { evil: bool, gender: Gender, decadent: bool, disguised: bool },
    MoonGoddess { evil: bool, decadent: bool, disguised: bool },
    AirGod { evil: bool, gender: Gender, decadent: bool, disguised: bool },
    WarGod { evil: bool, decadent: bool, disguised: bool },
    LoveGoddess { evil: bool, decadent: bool, disguised: bool },
    UnderworldGod { evil: bool, gender: Gender, decadent: bool, disguised: bool },
    WisdomAndKnowledgeGod { evil: bool, gender: Gender, decadent: bool, disguised: bool },
    HealingGod { evil: bool, gender: Gender, decadent: bool, disguised: bool },
    TradeGod { evil: bool, gender: Gender, decadent: bool, disguised: bool },
    LuckGoddess { evil: bool, decadent: bool, disguised: bool },
    NightGoddess { evil: bool, decadent: bool, disguised: bool },
    ThievesGod { evil: bool, decadent: bool, gender: Gender, disguised: bool },
}

impl Deity {
    /// Generate a random (culturally appropriate) deity.
    pub fn random(culture: &Culture) -> Self {
        fn choice(culture: &Culture, evil: bool, disguised: bool, decadent: bool) -> Deity {
            match 1.d20() + culture.modifier() {
                ..=1 => Deity::AncestorWorship { evil },
                2 => Deity::BeastGods { evil },
                3 => Deity::HuntingGod { evil, gender: Gender::random_biased(Gender::Male) },
                4 => Deity::Trickster { evil, gender: Gender::random_biased(Gender::Male) },
                ..=6 => Deity::EarthGoddess { decadent },
                ..=8 => Deity::AgriculturalGoddess { evil, decadent, disguised },
                ..=10 => Deity::RulingDeity { evil, gender: Gender::random(), decadent, disguised },
                11 => Deity::WaterGod { evil, gender: Gender::random(), decadent, disguised },
                12 => Deity::FireGod { evil, gender: Gender::random_biased(Gender::Male), decadent, disguised },
                13 => Deity::MoonGoddess { evil, decadent, disguised },
                14 => Deity::AirGod { evil, gender: Gender::random_biased(Gender::Male), decadent, disguised },
                15 => choice(culture, true, disguised, decadent),
                16 => Deity::WarGod { evil, decadent, disguised },
                17 => Deity::LoveGoddess { evil, decadent, disguised },
                18 => Deity::UnderworldGod { evil, gender: Gender::random(), decadent, disguised },
                19 => Deity::WisdomAndKnowledgeGod { evil, gender: Gender::random(), decadent, disguised },
                20 => Deity::HealingGod { evil, gender: Gender::random_biased(Gender::Female), decadent, disguised },
                21 => Deity::TradeGod { evil, gender: Gender::random_biased(Gender::Male), decadent, disguised },
                22 => Deity::LuckGoddess { evil, decadent, disguised },
                23 => Deity::NightGoddess { evil, decadent, disguised },
                24 => Deity::ThievesGod { evil, decadent, gender: Gender::random(), disguised },
                // Make 'decadent' or a 'decadent' into 'disguised evil' + 'decadent'.
                _ => if !decadent {
                    choice(culture, evil, disguised, true)
                } else {
                    choice(culture, true, true, decadent)
                }
            }
        }

        choice(culture, false, false, false)
    }
}
