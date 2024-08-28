use crate::{base_culture::BaseCulture, modifier::Modifiered, skill::environment::Environment};

/**
 Core culture info.
 */
pub struct Culture {
    base: BaseCulture,
    native_of: Environment,
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
     Get native environment.
     */
    pub fn native_of(&self) -> &Environment {
        &self.native_of
    }

    /**
     Generate a completely random culture.
     */
    pub fn random() -> Self {
        let base = BaseCulture::random();
        Self {
            native_of: base.random_native_of(),
            base: base,
        }
    }
}
