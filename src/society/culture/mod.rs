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

impl std::fmt::Display for CultureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::PRIMITIVE => "primitive",
            Self::NOMAD => "nomad",
            Self::BARBARIAN => "barbarian",
            Self::CIVILIZED => "civilized",
            _ => "decadent civilized"
        })
    }
}
