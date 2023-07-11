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
