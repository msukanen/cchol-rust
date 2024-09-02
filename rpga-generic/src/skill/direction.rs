use dicebag::{lo, DiceExt, HiLo};

/// Anything directional...
pub enum Direction {
    North,
    West,
    South,
    East,
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Generate left/right randomly.
    pub fn random_lr() -> Self {if lo!() {Self::Left} else {Self::Right}}
}
