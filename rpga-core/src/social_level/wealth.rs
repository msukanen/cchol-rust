/**
 Various wealth levels.
 */
pub enum Wealth {
    /// Someone utterly penniless.
    Destitute,
    /// Generally poor person.
    Poor,
    /// Comfortable/average wealth.
    Comfortable,
    /// Somewhat above-average wealth.
    WellToDo,
    /// Wealthy (or Extremely wealthy, if 'true').
    Wealthy(bool),
}
