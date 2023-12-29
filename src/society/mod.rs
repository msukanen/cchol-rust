pub mod culture;
pub mod status;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum NobleTitle {
    HETMAN, KNIGHT, PRINCE,
    BARONET, BARON, COUNT,
    SUBCHIEF, JARL, VISCOUNT,
    CHIEF, MARQUIS, DUKE,
    ARCHDUKE, ROYALPRINCE, KAHN,
    KING, HIGHKING, EMPEROR,
}

impl std::fmt::Display for NobleTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::HETMAN => "hetman",
            Self::KNIGHT => "knight",
            Self::PRINCE => "prince",
            Self::BARONET => "baronet",
            Self::BARON => "baron",
            Self::COUNT => "count",
            Self::SUBCHIEF => "subchief",
            Self::JARL => "jarl",
            Self::VISCOUNT => "viscount",
            Self::CHIEF => "chief",
            Self::MARQUIS => "marquis",
            Self::DUKE => "duke",
            Self::ARCHDUKE => "archduke",
            Self::ROYALPRINCE => "[royal] prince",
            Self::KAHN => "kahn",
            Self::KING => "king",
            Self::HIGHKING => "high-king",
            Self::EMPEROR => "emperor"
        })
    }
}
