pub mod culture;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum CultureType {
    PRIMITIVE,
    NOMAD,
    BARBARIAN,
    CIVILIZED,
    DECADENT,
    ANY,
}
