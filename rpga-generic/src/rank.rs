/// Skill etc. rank.
#[derive(Debug, Clone)]
pub struct Rank {
    value: i32,
}

impl From<i32> for Rank {
    /**
     Construct rank from given `value`. Actual value will be clamped to \[0..=20\].
     */
    fn from(value: i32) -> Self {
        Self { value: value.clamp(0, 20) }
    }
}

impl Rank {
    pub const ZERO: Self = Self { value: 0 };
}

impl PartialEq<i32> for Rank {
    fn eq(&self, other: &i32) -> bool {
        self.value.eq(other)
    }
}

impl PartialEq<i32> for &Rank {
    fn eq(&self, other: &i32) -> bool {
        self.value.eq(other)
    }
}

impl PartialEq<Rank> for i32 {
    fn eq(&self, other: &Rank) -> bool {
        other.eq(self)
    }
}

impl PartialEq<&Rank> for i32 {
    fn eq(&self, other: &&Rank) -> bool {
        other.eq(self)
    }
}
