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
}
