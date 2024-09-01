use std::sync::LazyLock;

use dicebag::DiceExt;
use rpga_generic::skill::{environment::Environment, Skill, Skilled};

use crate::{culture::Culture, skill::survival::{make_survival_native_of, make_survival_not_native_of, make_survival_urban, make_survival_wilds}};

static SKILLS_PRIMITIVE: LazyLock<Vec<Skill>> = LazyLock::new(|| {
    let mut skills = vec![];
    skills.push(make_survival_wilds(5.into()));
    skills.push(make_survival_urban(1.into()));
    skills
});
static SKILLS_NOMAD: LazyLock<Vec<Skill>> = LazyLock::new(|| {
    let mut skills = vec![];
    skills.push(make_survival_wilds(4.into()));
    skills.push(make_survival_urban(1.into()));
    skills
});
static SKILLS_BARBARIAN: LazyLock<Vec<Skill>> = LazyLock::new(|| {
    let mut skills = vec![];
    //TODO: survival native_of chooser somewhere else...
    skills.push(make_survival_native_of(5.into()));
    skills.push(make_survival_not_native_of(1.into()));
    skills
});
static SKILLS_CIVILIZED: LazyLock<Vec<Skill>> = LazyLock::new(|| {
    let mut skills = vec![];
    //TODO: survival native_of chooser somewhere else...
    skills.push(make_survival_native_of(2.into()));
    skills
});
static SKILLS_DECADENT_CIV: LazyLock<Vec<Skill>> = LazyLock::new(|| {
    let mut skills = vec![];
    skills.push(make_survival_urban(3.into()));
    skills.push(make_survival_wilds(1.into()));
    skills
});

/// Various culture levels.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BaseCulture {
    Primitive,
    Nomad,
    Barbarian,
    Civilized(bool)
}

impl BaseCulture {
    /// Generate a random culture level.
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

     **Params**
     * `base_culture` - base [culture][BaseCulture] reference.
     
     **Returns** one or other [Environment].
     */
    pub(crate) fn random_native_env(&self) -> Environment {
        match self {
            Self::Primitive        => Environment::Wilderness,
            Self::Nomad            => Environment::Wilderness,
            Self::Barbarian        => if 1.d3() == 1 {Environment::Urban} else {Environment::Wilderness},
            Self::Civilized(false) => if 1.d3() == 1 {Environment::Wilderness} else {Environment::Urban},
            _                      => Environment::Urban
        }
    }
}

impl PartialEq<Culture> for BaseCulture {
    fn eq(&self, other: &Culture) -> bool {
        self.eq(other.base())
    }
}

impl Skilled for BaseCulture {
    fn skills(&self) -> &Vec<Skill> {
        match self {
            BaseCulture::Primitive => &SKILLS_PRIMITIVE,
            BaseCulture::Nomad => &SKILLS_NOMAD,
            BaseCulture::Barbarian => &SKILLS_BARBARIAN,
            BaseCulture::Civilized(false) => &SKILLS_CIVILIZED,
            BaseCulture::Civilized(_) => &SKILLS_DECADENT_CIV,
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
