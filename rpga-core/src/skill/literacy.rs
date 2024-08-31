use std::ops::{Add, Sub};

/// Literacy modifier types.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LiteracyRate {
    Additive { rate: i32 },
    Fixed { rate: i32 },
}

impl Add<&LiteracyRate> for &LiteracyRate {
    type Output = LiteracyRate;
    /**
     Sum two rate (references) &mdash; see `Logic` below.

     **Params**
     * `rhs` - another [LiteracyRate].
     
     **Returns** either a sum or a choice.

     ## Logic
     If one is (but not both are) [LiteracyRate::Fixed] then that particular one is returned.

     If both are [LiteracyRate::Fixed] then the lowest of the two rates will be chosen and not added together.
 
     ```norun
       Additive{1} + Additive{2} → Additive{3}         // as expected.
       Additive + Fixed          → Fixed               // ignore Additive!
       Fixed1 + Fixed2           → min(Fixed1, Fixed2) // choose the lowest of the two.
     ```
     */
    fn add(self, rhs: &LiteracyRate) -> Self::Output {
        match self {
            LiteracyRate::Fixed { rate } => match rhs {
                LiteracyRate::Fixed { rate: rhs} => LiteracyRate::Fixed { rate: std::cmp::min(*rate, *rhs) },
                LiteracyRate::Additive {..} => self.clone()
            },
            LiteracyRate::Additive { rate } => match rhs {
                LiteracyRate::Fixed {..} => rhs.clone(),
                LiteracyRate::Additive { rate: rhs } => LiteracyRate::Additive { rate: rate + rhs }
            }
        }
    }
}

impl Add<i32> for &LiteracyRate {
    type Output = LiteracyRate;
    /**
     Adds `rhs` to [LiteracyRate].

     **Returns** a brand new [LiteracyRate].
     */
    fn add(self, rhs: i32) -> Self::Output {
        match self {
            LiteracyRate::Fixed { rate } => LiteracyRate::Fixed { rate: rate + rhs },
            LiteracyRate::Additive { rate } => LiteracyRate::Additive { rate: rate + rhs },
        }
    }
}

impl Sub<i32> for LiteracyRate {
    type Output = Self;
    /**
     Subtract `rhs` from [LiteracyRate].

     **Returns** a brand new [LiteracyRate].
     */
    fn sub(self, rhs: i32) -> Self::Output {
        match self {
            Self::Fixed { rate } => Self::Fixed { rate: rate - rhs },
            Self::Additive { rate } => Self::Additive { rate: rate - rhs },
        }
    }
}

#[cfg(test)]
mod literacy_rate_tests {
    use super::LiteracyRate;

    #[test]
    fn literacy_additional_sum_correctly() {
        let r1 = LiteracyRate::Additive { rate: 10 };
        let r2 = LiteracyRate::Additive { rate: 20 };
        assert_eq!(LiteracyRate::Additive { rate: 30 }, &r1 + &r2);
        assert_eq!(LiteracyRate::Additive { rate: 0 }, r1 - 10);
    }

    #[test]
    fn literacy_fixed_sum_correctly() {
        let r1 = LiteracyRate::Fixed { rate: 10 };
        let r2 = LiteracyRate::Fixed { rate: 20 };
        let r3 = LiteracyRate::Additive { rate: 30 };
        assert_eq!(LiteracyRate::Fixed { rate: 10 }, &r1 + &r2);
        assert_eq!(LiteracyRate::Fixed { rate: 20 }, &r2 + &r3);
    }
}
