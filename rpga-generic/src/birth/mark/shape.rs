use dicebag::{lo, DiceExt, HiLo};

/// Various potential birthmark shapes.
pub enum BirthmarkShape {
    Animal,//TODO: generate animal enum somewhere?
    Bat,
    Claw,
    CrescentMoon,
    Dragon,
    Eagle,
    Fish,
    Hand,
    Hawk,
    Skull,
    Sword,
    Whale,
}

impl BirthmarkShape {
    /// Generate a random birthmark shape.
    pub fn random() -> Self {
        match 1.d10() {
            ..=1 => Self::Dragon,
            2 => Self::Skull,
            3 => Self::Bat,
            4 => Self::Sword,
            5 => Self::Hand,
            6 => Self::CrescentMoon,
            7 => Self::Claw,
            8 => if lo!() {Self::Eagle} else {Self::Hawk},
            9 => if lo!() {Self::Fish} else {Self::Whale},
            _ => Self::Animal
        }
    }
}
