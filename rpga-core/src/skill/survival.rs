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

pub(crate) fn make_survival_native_of(rank: Rank) -> Skill {
    //TODO: native_of chooser elsewhere in the code...
    let mut s = Skill::from(("Survival: {<native_of>}", SkillType::Outdoors, rank));
    s.set_description("TODO: survival native_of");
    s
}

pub(crate) fn make_survival_not_native_of(rank: Rank) -> Skill {
    //TODO: native_of chooser elsewhere in the code...
    let mut s = Skill::from(("Survival: {<not_native_of>}", SkillType::Outdoors, rank));
    s.set_description("TODO: survival native_of");
    s
}
