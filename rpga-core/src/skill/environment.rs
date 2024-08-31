use dicebag::DiceExt;

use crate::base_culture::BaseCulture;

/**
 Environments.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Environment {
    Urban,
    Wilderness
}

impl Environment {
    /**
     Generate (pseudo)random native environment based on culture.
     Some cultures have only fixed native environment while others are more dynamic.

     **Params**
     * `base_culture` - base [culture][BaseCulture] reference.
     
     **Returns** one or other [Environment].
     */
    pub fn random_for(base_culture: &BaseCulture) -> Environment {
        match base_culture {
            BaseCulture::Primitive        => Environment::Wilderness,
            BaseCulture::Nomad            => Environment::Wilderness,
            BaseCulture::Barbarian        => if 1.d3() == 1 {Environment::Urban} else {Environment::Wilderness},
            BaseCulture::Civilized(false) => if 1.d3() == 1 {Environment::Wilderness} else {Environment::Urban},
            _                             => Environment::Urban
        }
    }
}
