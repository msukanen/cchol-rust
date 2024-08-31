use dicebag::DiceExt;

/// Some playable races.
#[derive(Debug, Clone)]
pub enum Race {
    Centaur,
    Elf,
    Faun,
    Dragonman,
    Dwarf,
    Goatman,
    HalfElf,
    HalfOrc,
    Halfling,
    Human,
    Minotaur,
    Orc,
    Reptileman,
    Satyr,
    Serpentman,
}

impl Race {
    /// **Returns** a random [Race].
    pub fn random() -> Self {
        match 1.d20() {
            ..=14 => Self::Human,
            _     => Self::random_nonhuman()
        }
    }

    /// **Returns** a random non-human [Race].
    pub fn random_nonhuman() -> Self {
        match 1.d20() {
            ..=4 => Self::Elf,
            ..=8 => Self::Dwarf,
            ..=11 => Self::Halfling,
            ..=15 => Self::HalfElf,
            16 => match 1.d10() {
                ..=4 => Self::Centaur,
                ..=7 => Self::Faun,
                8 => Self::Satyr,
                9 => Self::Goatman,
                _ => Self::Minotaur
            },
            17 => match 1.d6() {
                ..=3 => Self::Reptileman,
                ..=5 => Self::Serpentman,
                _ => Self::Dragonman
            },
            18 => Self::Orc,
            _  => Self::HalfOrc
        }
    }
}
