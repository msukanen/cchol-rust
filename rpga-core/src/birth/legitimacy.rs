use dicebag::DiceExt;
use rpga_traits::Modifiered;

use crate::{base_culture::BaseCulture, culture::Culture};

#[derive(Debug, Clone)]
pub enum BirthLegitimacy {
    Legitimate,
    Illegitimate(i32, &'static str),
}

impl BirthLegitimacy {
    /**
     Generate random birth legitimacy case.

     **Params**
     * `culture` - one or other [Culture].
      
     **Returns** [BirthLegitimacy].
     */
    pub fn random(culture: &Culture) -> Self {
        fn illegitmacy_reason(culture: &Culture) -> &'static str {
            static R1: &'static str = "Mother was a common prostitute and unmarried.";//TODO: reasons...
            static R2: &'static str = "Mother was raped - remained unmarried. Father's identity is known.";
            static R3: &'static str = "Mother was raped - remained unmarried. Father's identity is unknown.";
            static R4: &'static str = "Mother was unmarried and father's identity is unknown.";
            static R5: &'static str = "Mother was unmarried but father's identity is known.";
            static R6: &'static str = "Mother was a courtesan (prostitute to Nobility). Father's identity is unknown.";
            static R7: &'static str = "Mother was a courtesan (prostitute to Nobility). Father's identity is known.";
            match 1.d20() + culture.modifier() {
                ..=12 => R1,
                ..=14 => if 1.d100() < 16 {R2} else {R3},
                ..=23 => if 1.d100() < 51 {R4} else {R5},
                _     => if 1.d100() < 51 {R6} else {R7}
            }
        }

        match culture.base() {
            BaseCulture::Primitive => if 1.d20() == 20 {
                Self::Illegitimate(1.d4(), illegitmacy_reason(culture))
            } else { Self::Legitimate },

            _ => if 1.d20() + culture.modifier() >= 19 {
                Self::Illegitimate(1.d4(), illegitmacy_reason(culture))
            } else { Self::Legitimate }
        }
    }
}
