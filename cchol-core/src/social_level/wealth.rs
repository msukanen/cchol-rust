use dicebag::DiceExt;
use rpga_traits::Modifiered;
use crate::culture::Culture;

/// Various wealth levels.
pub enum WealthRank {
    /// Someone utterly penniless.
    Destitute,
    /// Generally poor person.
    Poor,
    /// Comfortable/average wealth.
    Comfortable,
    /// Somewhat above-average wealth.
    WellToDo,
    /// Wealthy (or Extremely wealthy, if 'true').
    Wealthy(bool),
}

impl WealthRank {
    /**
     Generate a random, culture & society based, wealth rank.

     **Params**
     * *cumod* - ***CuMod***.
     * *timod* - ***TiMod***.
     */
    pub(crate) fn random(culture: &Culture, timod: i32) -> Self {
        fn choice(cumod: i32, timod: i32) -> WealthRank {
            match 1.d100() + cumod + timod {
                ..=12 => WealthRank::Destitute,
                ..=40 => WealthRank::Poor,
                ..=84 => WealthRank::Comfortable,
                85 => choice(0, timod),// reroll w/o CuMod
                ..=94 => WealthRank::WellToDo,
                ..=98 => WealthRank::Wealthy(false),
                _ => choice(cumod, timod)
            }
        }

        let mut rank = choice(culture.modifier(), timod);
        // Some 'Wealthy' are 'Extremely Wealthy'; let the dice decide if so...
        match rank {
            Self::Wealthy(_) => if 1.d100() <= 1 + timod {
                rank = Self::Wealthy(true)
            },
            _ => ()
        };
        rank
    }

    /// Generate (pseudo)random survival modifier for the [WealthRank].
    pub(crate) fn random_survival_mod(&self) -> i32 {
        match self {
            Self::Destitute => 1.d2(),
            Self::Poor => 1,
            Self::Comfortable => 0,
            Self::WellToDo => -1,
            Self::Wealthy(_) => -1.d2(),
        }
    }
}

impl Modifiered for WealthRank {
    /// Get ***SolMod***.
    fn modifier(&self) -> i32 {
        match self {
            Self::Destitute => -3,
            Self::Poor => -1,
            Self::Comfortable => 0,
            Self::WellToDo => 2,
            Self::Wealthy(false) => 4,
            Self::Wealthy(_) => 7,
        }
    }
}

/**
 Wealth data belongs here...
 */
pub struct Wealth {
    rank: WealthRank,
    survival_mod: i32,
}

impl Wealth {
    /**
     Generate (randomly) culture and society appropriate wealth level.

     **Params**
     * *cumod* - ***CuMod***.
     * *timod* - ***TiMod***.
     */
    pub fn random(culture: &Culture, timod: i32) -> Self {
        let rank = WealthRank::random(culture, timod);

        Self { survival_mod: rank.random_survival_mod(), rank }
    }

    /// Get survival skill modifier that is associated with the wealth level.
    pub fn survival_mod(&self) -> i32 {
        self.survival_mod
    }
}

impl Modifiered for Wealth {
    /// Get ***SolMod***.
    fn modifier(&self) -> i32 {
        self.rank.modifier()
    }
}
