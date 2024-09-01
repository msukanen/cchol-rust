use dicebag::{lo, HiLo, DiceExt};

/// Birth orders.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BirthOrder {
    First,
    Second,
    Middle,
    SecondToLast,
    Last,
}

impl BirthOrder {
    /// Generate random birth order (loosely based on number of siblings).
    pub fn random(num_siblings: usize) -> Self {
        match num_siblings {
            ..=0 => Self::First,
            ..=1 => if lo!() {Self::First} else {Self::Second},
            ..=2 => match 1.d4() {
                ..=1 => Self::First,
                ..=2 => Self::Second,
                _    => Self::Middle,
            },
            _ => match 1.d20() {
                ..=2  => Self::First,
                ..=10 => Self::Second,
                ..=16 => Self::Middle,
                ..=18 => Self::SecondToLast,
                _     => Self::Last
            }
        }
    }
}
