use dicebag::DiceExt;

/// Body locations.
pub enum BodyLocation {
    Abdomen,
    Arm { left: bool, right: bool },
    Back,
    Buttock { left: bool, right: bool },
    Chest,
    Face,
    Foot { left: bool, right: bool },
    Genitals,
    Hand { left: bool, right: bool },
    Head,
    Leg { left: bool, right: bool },
    FingerL { thumb: bool, index: bool, other: i32 },
    FingerR { thumb: bool, index: bool, other: i32 },
}

impl BodyLocation {
    /// Generate a random body location for various purposes.
    pub fn random() -> Self {
        match 1.d20() {
            ..=1 => Self::Foot { left: false, right: true },
            2 => Self::Foot { left: true, right: false },
            3 => Self::Leg { left: false, right: true },
            4 => Self::Leg { left: true, right: false },
            ..=6 => Self::Abdomen,
            ..=8 => Self::Buttock { left: true, right: true },
            9 => Self::Back,
            ..=13 => Self::Chest,
            14 => Self::Arm { left: false, right: true },
            15 => Self::Arm { left: true, right: false },
            16 => Self::Hand { left: false, right: true },
            17 => Self::Hand { left: true, right: false },
            18 => Self::Head,
            _  => Self::Face
        }
    }
}
