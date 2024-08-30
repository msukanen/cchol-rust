use crate::skill::{literacy::LiteracyRate, Skill};

/**
 A trait for anything with "skills".
 */
pub trait Skilled {
    /**
     Get the associated skills.
     */
    fn skills(&self) -> &Vec<Skill>;
}

/**
 A trait for anything with "literacy".
 */
pub trait Literated {
    fn literacy(&self) -> &Vec<LiteracyRate>;
}
