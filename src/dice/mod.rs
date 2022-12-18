use rand::Rng;
use num::{ Float, NumCast };

/**
 Dice extensions.
 */
pub trait DiceExt {
    /// A `sides`-die.
    fn d(&self, sides:u8) -> Self;
    /// A `sides`-die w/ modifier.
    fn d_(&self, sides:u8, modifier:Self) -> Self;
    /// A d6.
    fn d6(&self) -> Self;
    /// A d6 w/ modifier.
    fn d6_(&self, modifier:Self) -> Self;
    /// A d100.
    fn d100(&self) -> Self;
    /// A d100 w/ modifier.
    fn d100_(&self, modifier:Self) -> Self;
    /// If chance on `d100` matches `of` then return self, otherwise return None.
    fn chance(&self, of:i32) -> Option<i32>;
}

/**
 Throw given `num` of dice, each with x `sides`.
*/
fn any_i32(num: i32, sides:u8) -> i32 {
    let mut result: i32 = 0;
    for _ in 0..num {
        result += rand::thread_rng().gen_range(1..=(sides as i32));
    }
    result
}

impl DiceExt for i32 {
    fn d(&self, sides:u8) -> Self { any_i32(*self, sides)}
    fn d_(&self, sides:u8, modifier:Self) -> Self { self.d(sides) + modifier}
    fn d6(&self) -> Self { self.d(6)}
    fn d6_(&self, modifier:Self) -> Self {self.d6() + modifier}
    fn d100(&self) -> Self { self.d(100)}
    fn d100_(&self, modifier:Self) -> Self {self.d100() + modifier}
    fn chance(&self, of:i32) -> Option<i32> {
        if 3.d6() as Self <= *self {Some(of)} else {None}
    }
}

/**
 Value variators.
 */
pub trait VarianceExt {
    /**
     Take a number and alter it by up to (or less, of course) +/- X%.
    */
    fn delta(&self, percentage:i32) -> Self;
}

/**
 Take a number and alter it by up to (or less, of course) +/- X%.
 */
fn delta<T: Float>(original:&T, percentage:i32) -> T {
    let r = rand::thread_rng().gen_range(0..=(percentage*2)) - percentage;
    let d = 1.0 + 0.01 * r as f64;
    *original * NumCast::from(d).unwrap()
}

impl VarianceExt for f32 {
    fn delta(&self, percentage:i32) -> Self { delta::<Self>(self, percentage) }
}

impl VarianceExt for f64 {
    fn delta(&self, percentage:i32) -> Self { delta::<Self>(self, percentage) }
}
