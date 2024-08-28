use dicebag::DiceExt;

/**
 Various culture levels.
 */
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BaseCulture {
    Primitive,
    Nomad,
    Barbarian,
    Civilized(bool)
}

impl BaseCulture {
    /**
     Generate a random culture level.
     */
    pub fn random() -> Self {
        match 1.d10() {
            ..=1 => Self::Primitive,
            2|3  => Self::Nomad,
            ..=6 => Self::Barbarian,
            ..=9 => Self::Civilized(false),
            _    => Self::Civilized(true),
        }
    }
}

#[cfg(test)]
mod culture_tests {
    use crate::base_culture::BaseCulture;

    #[test]
    fn culture_ord_works() {
        assert!(BaseCulture::Primitive < BaseCulture::Nomad);
        assert!(BaseCulture::Civilized(false) < BaseCulture::Civilized(true));
    }
}
