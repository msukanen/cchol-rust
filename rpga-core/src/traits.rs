/**
 A trait for anything with "modifier" payload.
 */
pub trait Modifiered {
    /**
     Get the associated modifier value.
     */
    fn modifier(&self) -> i32;
}

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
