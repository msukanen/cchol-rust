pub mod status;
pub mod nobility;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum WealthLevel {
    DESTITUTE,
    POOR,
    AVERAGE,
    COMFORTABLE,
    WEALTHY,
    RICH,
}
