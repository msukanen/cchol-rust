use rpga_traits::{Described, Named};

use crate::{rank::Rank, Ranked};

/// Various skill types.
#[derive(Debug, Clone)]
pub enum SkillType {
    Magic,
    Outdoors,
    Profession,
    Stealth,
    Unspecified,
}

/// Skill data lives here.
#[derive(Debug, Clone)]
pub struct Skill {
    name: String,
    rank: Rank,
    description: String,
    r#type: SkillType,
}

impl Ranked for Skill {
    fn rank(&self) -> &Rank {
        &self.rank
    }
}

impl Named for Skill {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Described for Skill {
    fn description(&self) -> &str {
        &self.description
    }
}

impl Skill {
    /// Create a blank skill with no name, description or rank.
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), rank: Rank::ZERO, description: String::from(""), r#type: SkillType::Unspecified }
    }

    /// Set rank.
    pub fn set_rank(&mut self, rank: Rank) -> &mut Self {
        self.rank = rank;
        self
    }

    /// Set description.
    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = description.to_string();
        self
    }

    /// Set type.
    pub fn set_type(&mut self, r#type: SkillType) -> &mut Self {
        self.r#type = r#type;
        self
    }
}

impl From<&str> for Skill {
    /// Generate an unranked and descriptionless skill with just a `name`.
    fn from(name: &str) -> Self {
        Self::from((name, Rank::ZERO))
    }
}

impl From<(&str, i32)> for Skill {
    /// Generate a ranked (but descriptionless) skill from name/rank tuple.
    fn from(value: (&str, i32)) -> Self {
        Self::from((value.0, Rank::from(value.1)))
    }
}

impl From<(&str, Rank)> for Skill {
    /// Generate a ranked (but descriptionless) skill from name/rank tuple.
    fn from(value: (&str, Rank)) -> Self {
        Self { name: value.0.to_string(), rank: value.1, description: String::from(""), r#type: SkillType::Unspecified }
    }
}

impl From<(&str, SkillType)> for Skill {
    /// Generate an unranked skill with just name and type.
    fn from(value: (&str, SkillType)) -> Self {
        Self { name: value.0.to_string(), rank: Rank::ZERO, description: String::from(""), r#type: value.1 }
    }
}

impl From<(&str, SkillType, i32)> for Skill {
    /// Generate a ranked skill with name, type and rank.
    fn from(value: (&str, SkillType, i32)) -> Self {
        Self::from((value.0, value.1, Rank::from(value.2)))
    }
}

impl From<(&str, i32, SkillType)> for Skill {
    /// Generate a ranked skill with name, type and rank.
    fn from(value: (&str, i32, SkillType)) -> Self {
        Self::from((value.0, value.2, value.1))
    }
}

impl From<(&str, SkillType, Rank)> for Skill {
    /// Generate a ranked skill with name, type and rank.
    fn from(value: (&str, SkillType, Rank)) -> Self {
        Self { name: value.0.to_string(), rank: value.2, description: String::from(""), r#type: value.1 }
    }
}

impl From<(&str, Rank, SkillType)> for Skill {
    /// Generate a ranked skill with name, type and rank.
    fn from(value: (&str, Rank, SkillType)) -> Self {
        Self::from((value.0, value.2, value.1))
    }
}

/// A trait for anything with "skills".
pub trait Skilled {
    /// Get the associated skills.
    fn skills(&self) -> &Vec<Skill>;
}

#[cfg(test)]
mod skill_tests {
    use rpga_traits::{Described, Named};

    use crate::Ranked;

    use super::Skill;

    #[test]
    fn builder_model_works() {
        let mut s = Skill::new("A Skill");
        s   .set_rank(5.into())
            .set_description("A description!");
        assert_eq!("A Skill", s.name());
        assert_eq!(5, s.rank());
        assert_eq!("A description!", s.description());
    }

    #[test]
    fn from_name_works() {
        let s = Skill::from("A Skill");
        assert_eq!("A Skill", s.name());
        assert_eq!(0, s.rank());
        assert_eq!("", s.description());
    }

    #[test]
    fn from_name_and_rank_works() {
        let s = Skill::from(("A Skill", 5));
        assert_eq!("A Skill", s.name());
        assert_eq!(5, s.rank());
        assert_eq!("", s.description());
    }
}
