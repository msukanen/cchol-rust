/**
 A trait for anything with "modifier" payload.
 */
pub trait Modifiered {
    /**
     Get the associated modifier value.
     */
    fn modifier(&self) -> i32;
}
