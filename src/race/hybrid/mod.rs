pub mod halfelf;
pub mod halforc;

pub trait HybridRace {
    /// Raised by `Human`s?
    fn raised_by_humans(&self) -> bool;
}
