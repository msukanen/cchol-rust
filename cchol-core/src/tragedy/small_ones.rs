use dicebag::DiceExt;

use crate::misc::seriouswound::SeriousWound;

pub enum ChildDies {
    Accident,
    Disease,
    Fire,
    SomeonesActions { other_person: Other, death_situation: DeathSituation }
}

impl ChildDies {
    /// Generate a tragic death situation.
    pub fn random() -> Self {
        match 1.d6() {
            ..=2 => Self::Accident,
            3    => Self::Fire,
            ..=5 => Self::Disease,
            _    => Self::SomeonesActions { other_person: Other::random(), death_situation: DeathSituation::random() }
        }
    }
}

pub enum TragicDeathReason {
    Disease,
    Fire,
    SomeonesActions(Other),
    War,
}

pub enum PossessionVanished {
    Lost,
    Stolen,
    StolenAndFaked
}

impl PossessionVanished {
    /// Generate random possession loss/theft reason.
    pub fn random() -> Self {
        match 1.d6() {
            ..=3 => Self::Lost,
            ..=5 => Self::Stolen,
            _    => Self::StolenAndFaked
        }
    }
}

pub enum SevereInjury {
    Accident(SeriousWound),
    Fire(SeriousWound),
    AttackedByAnimal(SeriousWound),
    AttackedBySomeone { wound: SeriousWound, attacker: Other },
}

impl SevereInjury {
    /// Generate random injury related tragedy thingy.
    pub fn random() -> Self {
        match 1.d8() {
            ..=4 => Self::Accident(SeriousWound::random()),
            5 => Self::Fire(SeriousWound::random()),
            6 => Self::AttackedByAnimal(SeriousWound::random()),
            _ => Self::AttackedBySomeone { wound: SeriousWound::random(), attacker: Other::random() }
        }
    }
}
