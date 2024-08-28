use crate::traits::{Described, Named, Ranked};

pub mod environment;

pub struct Skill {
    name: String,
    rank: i32,
    description: String,
}

impl Ranked for Skill {
    fn rank(&self) -> i32 {
        self.rank
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
