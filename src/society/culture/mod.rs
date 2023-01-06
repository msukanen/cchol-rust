pub mod culture;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum CultureType {
    PRIMITIVE,
    NOMAD,
    BARBARIAN,
    CIVILIZED,
    DECADENT,
    ANY,
}
