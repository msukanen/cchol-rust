use crate::skill::literacy::LiteracyRate;

/**
 A trait for anything with "literacy".
 */
pub trait Literated {
    fn literacy(&self) -> &Vec<LiteracyRate>;
}
