use legitimacy::BirthLegitimacy;
use rpga_traits::Modifiered;

pub mod legitimacy;

/**
 Birth and everything associated with it.
 */
pub struct Birth {
    legitimacy: BirthLegitimacy,
}

impl Modifiered for Birth {
    /// Get ***BiMod***.
    fn modifier(&self) -> i32 {
        match self.legitimacy {
            BirthLegitimacy::Legitimate => 0,
            BirthLegitimacy::Illegitimate(x,_) => x
        }
    }
}

impl Birth {
    /// Get birth illegitimacy reason, if such exists.
    pub fn illegitmacy_reason(&self) -> Option<&'static str> {
        match self.legitimacy {
            BirthLegitimacy::Legitimate => None,
            BirthLegitimacy::Illegitimate(_,r) => Some(r)
        }
    }
}
