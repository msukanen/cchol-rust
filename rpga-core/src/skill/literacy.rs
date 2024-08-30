use std::ops::{Add, Sub};

pub enum LiteracyRate {
    Fixed { rate: i32 },
    Additive { rate: i32 },
}

impl Add<LiteracyRate> for LiteracyRate {
    type Output = Self;
    fn add(self, rhs: LiteracyRate) -> Self::Output {
        match self {
            Self::Fixed { rate } => match rhs {
                Self::Fixed { rate: rhs} => Self::Fixed { rate: std::cmp::min(rate, rhs) },
                _ => self
            },
            Self::Additive { rate } => match rhs {
                Self::Fixed {..} => rhs,
                Self::Additive { rate: rhs } => Self::Additive { rate: rate + rhs }
            }
        }
    }
}

impl Add<i32> for LiteracyRate {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        match self {
            Self::Fixed { rate } => Self::Fixed { rate: rate + rhs },
            Self::Additive { rate } => Self::Additive { rate: rate + rhs },
        }
    }
}

impl Sub<i32> for LiteracyRate {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        match self {
            Self::Fixed { rate } => Self::Fixed { rate: rate - rhs },
            Self::Additive { rate } => Self::Additive { rate: rate - rhs },
        }
    }
}
