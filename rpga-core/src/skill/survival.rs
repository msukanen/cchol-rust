use rpga_rank::rank::Rank;

use super::{Skill, SkillType};

pub fn make_survival_urban(rank: Rank) -> Skill {
    let mut s = Skill::from(("Survival: Urban", SkillType::Outdoors, rank));
    s.set_description("Urban survival");
    s
}

pub fn make_survival_wilds(rank: Rank) -> Skill {
    let mut s = Skill::from(("Survival: Wilderness", SkillType::Outdoors, rank));
    s.set_description("Wilderness survival");
    s
}
