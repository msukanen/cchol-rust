pub mod gender;
pub mod rank;

/**
 A trait for anything with "rank".
 */
pub trait Ranked {
    /**
     Get the associated rank value.
     */
    fn rank(&self) -> &Rank;
}
