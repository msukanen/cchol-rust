use dicebag::DiceExt;

/// Genders.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female
}

impl Gender {
    /**
     Generate gender in random.

     The roll is slightly biased, as per
        info found at https://en.wikipedia.org/wiki/Sex_ratio
     */
    pub fn random() -> Self {
        match 1.d(1934) {
            ..=1000 => Self::Male,
            _       => Self::Female
        }
    }

    /**
     Generate gender in random.

     **Params**
     * `bias` - result will clearly lean toward this.
     
     **Returns** some [Gender].
     */
    pub fn random_biased(bias: Self) -> Self {
        if 1.d20() < 9 {
            bias.opposite()
        } else {
            bias
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Male => Self::Female,
            Self::Female => Self::Male
        }
    }
}
