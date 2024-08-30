use crate::skill::{literacy::LiteracyRate, Skill};

/**
 A trait for anything with a "name".
 */
pub trait Named {
    /**
     Get the associated name value.
     */
    fn name(&self) -> &str;
}

/**
 A trait for anything with a "description".
 */
pub trait Described {
    /**
     Get the associated description.
     */
    fn description(&self) -> &str;
}

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
