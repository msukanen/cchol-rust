use rpga_modifier::Modifiered;

use crate::{base_culture::BaseCulture, skill::environment::Environment};

/**
 Core culture info.
 */
pub struct Culture {
    base: BaseCulture,
    native_of: Environment,
}

impl PartialEq<BaseCulture> for Culture {
    fn eq(&self, other: &BaseCulture) -> bool {
        self.base.eq(other)
    }
}

impl Modifiered for Culture {
    /**
     Get ***CuMod***.
     */
    fn modifier(&self) -> i32 {
        match self.base {
            BaseCulture::Primitive => -3,
            BaseCulture::Nomad => 0,
            BaseCulture::Barbarian => 2,
            BaseCulture::Civilized(false) => 4,
            _ => 7
        }
    }
}

impl Culture {
    /**
     Get the native environment.
     */
    pub fn native_of(&self) -> &Environment {
        &self.native_of
    }

    /**
     Get the base culture.
     */
    pub fn base(&self) -> &BaseCulture {
        &self.base
    }

    /**
     Generate a completely random culture.
     */
    pub fn random() -> Self {
        let base = BaseCulture::random();
        Self {
            native_of: Environment::random_for(&base),
            base,
        }
    }
}

impl From<BaseCulture> for Culture {
    /**
     Derive [Culture] from [BaseCulture]. Native environment will be (pseudo)randomized.
     */
    fn from(base: BaseCulture) -> Self {
        Self { native_of: Environment::random_for(&base), base }
    }
}

impl From<&BaseCulture> for Culture {
    /**
     Derive [Culture] from [BaseCulture]. Native environment will be (pseudo)randomized.
     */
    fn from(base: &BaseCulture) -> Self {
        Self { base: base.clone(), native_of: Environment::random_for(base) }
    }
}

impl From<(BaseCulture, Environment)> for Culture {
    /**
     Derive [Culture] from [BaseCulture] and a specific [Environment].

     **NOTE:** This is the only way to make e.g. "Urban Primitive"&mdash;primitives are natively wilderness-only.
     */
    fn from(value: (BaseCulture, Environment)) -> Self {
        Self { base: value.0, native_of: value.1 }
    }
}

#[cfg(test)]
mod culture_tests {
    use crate::{base_culture::BaseCulture, skill::environment::Environment};

    use super::Culture;

    #[test]
    fn from_base_works() {
        let c = Culture::from(BaseCulture::Barbarian);
        assert_eq!(&BaseCulture::Barbarian, c.base());
    }

    #[test]
    fn from_base_and_env_works() {
        let c = Culture::from((BaseCulture::Civilized(false), Environment::Wilderness));
        assert_eq!(&BaseCulture::Civilized(false), c.base());
        assert_eq!(&Environment::Wilderness, c.native_of());
    }
}
