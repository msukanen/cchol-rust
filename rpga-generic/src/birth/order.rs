/// Birth orders.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BirthOrder {
    First,
    Second,
    Middle,
    SecondToLast,
    Last,
}
