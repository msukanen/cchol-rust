use dicebag::DiceExt;

use crate::skill::environment::Environment;

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

    /**
     Generate (pseudo)random native environment based on culture.
     Some cultures have only fixed native environment while others are more dynamic.
     */
    pub fn random_native_of(&self) -> Environment {
        match self {
            Self::Primitive        => Environment::Wilderness,
            Self::Nomad            => Environment::Wilderness,
            Self::Barbarian        => if 1.d3() == 1 {Environment::Urban} else {Environment::Wilderness},
            Self::Civilized(false) => if 1.d3() == 1 {Environment::Wilderness} else {Environment::Urban},
            _                      => Environment::Urban
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
