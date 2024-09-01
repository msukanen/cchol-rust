use rank::Rank;

pub mod gender;
pub mod rank;
pub mod skill;
pub mod birth;

/**
 A trait for anything with "rank".
 */
pub trait Ranked {
    /**
     Get the associated rank value.
     */
    fn rank(&self) -> &Rank;
}
