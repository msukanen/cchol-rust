use legitimacy::BirthLegitimacy;
use place::PlaceOfBirth;
use rpga_traits::Modifiered;

use crate::culture::Culture;

pub mod legitimacy;
pub mod place;
pub mod unusual;

/// Birth and everything associated with it.
pub struct Birth {
    legitimacy: BirthLegitimacy,
    place: PlaceOfBirth
}

impl Birth {
    /// Get birth illegitimacy reason, if such exists.
    pub fn illegitmacy_reason(&self) -> Option<&'static str> {
        match self.legitimacy {
            BirthLegitimacy::Legitimate => None,
            BirthLegitimacy::Illegitimate(_,r) => Some(r)
        }
    }

    /// Get ***LegitMod***.
    pub fn legit_mod(&self) -> i32 {
        self.legitimacy.modifier()
    }

    /// Generate random birth condition(s).
    pub fn random(culture: &Culture) -> Self {
        let legitimacy = BirthLegitimacy::random(culture);
        Self { place: PlaceOfBirth::random(culture, legitimacy.modifier()), legitimacy }
    }

    /// Get place of birth.
    pub fn place(&self) -> &PlaceOfBirth {
        &self.place
    }
}
