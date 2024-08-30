use dicebag::DiceExt;

use crate::{base_culture::BaseCulture, culture::Culture};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Title {
    Hetman,
    Knight,
    Prince,
    Baronet,
    Baron,
    Count,
    Subchieftain,
    Jarl,
    Viscount,
    Chieftain,
    Marquis,
    Duke,
    Archduke,
    RoyalPrince,
    Kahn,
    King,
    HighKing,
    Emperor,
}

impl Title {
    /**
     Generate a random noble title.

     **Params**
     * *culture* - some [Culture] reference.

     **Returns** a noble [title][Title].
     */
    pub fn random(culture: &Culture) -> Self {
        match culture.base() {
            BaseCulture::Primitive => match 1.d100() {
                ..=1 => Self::HighKing,
                ..=30 => Self::Chieftain,
                _ => Self::Subchieftain
            },

            BaseCulture::Nomad => match 1.d100() {
                ..=10 => Self::Kahn,
                ..=40 => Self::Chieftain,
                ..=80 => Self::Subchieftain,
                _ => Self::Hetman
            },

            BaseCulture::Barbarian => match 1.d100() {
                ..=2 => Self::HighKing,
                ..=15 => Self::King,
                ..=25 => Self::RoyalPrince,
                ..=45 => Self::Chieftain,
                ..=60 => Self::Jarl,
                ..=70 => Self::Subchieftain,
                ..=75 => Self::Baron,
                ..=80 => Self::Prince,
                _ => Self::Hetman
            },

            BaseCulture::Civilized(_) => match 1.d100() {
                ..=1 => Self::Emperor,
                ..=5 => Self::King,
                ..=15 => Self::RoyalPrince,
                ..=20 => Self::Archduke,
                ..=25 => Self::Duke,
                ..=35 => Self::Marquis,
                ..=50 => Self::Viscount,
                ..=60 => Self::Count,
                ..=75 => Self::Baron,
                ..=78 => Self::Baronet,
                ..=90 => Self::Prince,
                _ => Self::Knight
            }
        }
    }

    /**
     Generate (pseudo)random ***TiMod*** based on [Title].

     **Params**
     * *prince_parent_timod* - parents' ***TiMod*** if self is [Title::Prince], otherwise ignored.
     
     **Returns** ***TiMod***.
     */
    pub(crate) fn random_timod(&self, prince_parent_timod: Option<&i32>) -> i32 {
        match self {
            Self::Archduke |
            Self::RoyalPrince => 4.d10(),
            Self::Baron => 2.d10(),
            Self::Baronet => 2.d8(),
            Self::Chieftain |
            Self::Count |
            Self::Jarl => 3.d6(),
            Self::Duke => 4.d8(),
            Self::Emperor => 60,
            Self::Hetman => 1.d6(),
            Self::HighKing => 5.d10(),
            Self::Kahn => 5.d8(),
            Self::King => 39,
            Self::Knight |
            Self::Subchieftain => 2.d6(),
            Self::Marquis => 3.d10(),
            Self::Prince => match prince_parent_timod {
                Some(t) => ((t * 1.d10() * 10) as f64 / 100.0) as i32,
                _ => 4.d10()
            },
            Self::Viscount => 3.d8()
        }
    }
}

#[cfg(test)]
mod title_tests {
    use super::Title;

    #[test]
    fn prince_random_timod_works() {
        let prince_parent_title = Title::Duke;
        let parent_timod = prince_parent_title.random_timod(None);
        let prince = Title::Prince;
        for _ in 0..=10_000 {
            let prince_timod = prince.random_timod(Some(&parent_timod));
            assert!(prince_timod <= parent_timod);
        }
    }
}
