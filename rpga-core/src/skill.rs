use rpga_rank::{rank::Rank, Ranked};

use crate::traits::{Described, Named};

pub mod environment;

pub struct Skill {
    name: String,
    rank: Rank,
    description: String,
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
    /**
     Create a blank skill with no name, description nor rank.
     */
    pub fn new() -> Self {
        Self { name: String::from(""), rank: Rank::ZERO, description: String::from("") }
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn set_rank(&mut self, rank: i32) -> &mut Self {
        self.rank = Rank::from(rank);
        self
    }

    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = description.to_string();
        self
    }
}

impl From<&str> for Skill {
    /**
     Generate an unranked and descriptionless skill with just a `name`.
     */
    fn from(name: &str) -> Self {
        Self { name: name.to_string(), rank: Rank::ZERO, description: String::from("") }
    }
}

impl From<(&str, i32)> for Skill {
    /**
     Generate a ranked (but descriptionless) skill from name/rank tuple.
     */
    fn from(value: (&str, i32)) -> Self {
        Self { name: value.0.to_string(), rank: Rank::from(value.1), description: String::from("") }
    }
}

#[cfg(test)]
mod skill_tests {
    use rpga_rank::Ranked;

    use crate::traits::{Described, Named};

    use super::Skill;

    #[test]
    fn builder_model_works() {
        let mut s = Skill::new();
        s   .set_name("A Skill")
            .set_rank(5)
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
