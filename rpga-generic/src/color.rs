use dicebag::{lo, DiceExt, HiLo};

/// Color tint.
pub enum Tint {
    Light,// a.k.a. "pastel" in some cases.
    Dark
}

impl Tint {
    /// Generate a random tint.
    pub fn random() -> Self {
        if lo!() { Self::Dark } else { Self::Light }
    }
}

/// Various colors.
pub enum Color {
    Red(Option<Tint>),
    Crimson(Option<Tint>),
    Scarlet(Option<Tint>),
    BloodRed(Option<Tint>),
    RedOrange(Option<Tint>),
    SunsetOrange(Option<Tint>),
    Orange(Option<Tint>),
    YellowOrange(Option<Tint>),
    Yellow(Option<Tint>),
    YellowGreen(Option<Tint>),
    Citrine(Option<Tint>),
    Green(Option<Tint>),
    BlueGreen(Option<Tint>),
    Aquamarine(Option<Tint>),
    Tourquoise(Option<Tint>),
    Blue(Option<Tint>),
    BlueViolet(Option<Tint>),
    RoyalBlue(Option<Tint>),
    Violet(Option<Tint>),
    Purple(Option<Tint>),
    Lavender(Option<Tint>),
    RedViolet(Option<Tint>),
    Magenta(Option<Tint>),
    HotPink(Option<Tint>),
    Pink(Option<Tint>),
    White,
    SnowWhite,
    OffWhite,
    Ivory,
    Black,
    Ebony,
    TrueBlack,
    VantaBlack,
    Gray(Option<Tint>),
    Maroon(Option<Tint>),
    ReddishBrown(Option<Tint>),
    PurplishBrown(Option<Tint>),
    Silver,
    Gold,
    Platinum,
}

impl Color {
    /// Generate a random color/tint combo.
    pub fn random() -> Self {
        fn choice(tint: Option<Tint>) -> Color {
            match 1.d20() {
                ..=1 => match 1.d4() {
                    ..=1 => Color::Red(tint),
                    2 => Color::Crimson(tint),
                    3 => Color::Scarlet(tint),
                    _ => Color::BloodRed(tint)
                },
                2 => match 1.d2() {
                    ..=1 => Color::RedOrange(tint),
                    _    => Color::SunsetOrange(tint)
                },
                3 => Color::Orange(tint),
                4 => Color::YellowOrange(tint),
                5 => Color::Yellow(tint),
                6 => match 1.d2() {
                    ..=1 => Color::YellowGreen(tint),
                    _    => Color::Citrine(tint)
                },
                7 => Color::Green(tint),
                8 => match 1.d3() {
                    ..=1 => Color::BlueGreen(tint),
                    2 => Color::Aquamarine(tint),
                    _ => Color::Tourquoise(tint)
                },
                9 => Color::Blue(tint),
                10 => match 1.d2() {
                    ..=1 => Color::BlueViolet(tint),
                    _    => Color::RoyalBlue(tint)
                },
                11 => match 1.d3() {
                    ..=1 => Color::Violet(tint),
                    2 => Color::Purple(tint),
                    _ => Color::Lavender(tint)
                },
                12 => match 1.d3() {
                    ..=1 => Color::RedViolet(tint),
                    2 => Color::Magenta(tint),
                    _ => Color::HotPink(tint)
                },
                13 => Color::Pink(tint),
                14 => match 1.d4() {
                    ..=1 => Color::White,
                    2 => Color::SnowWhite,
                    3 => Color::OffWhite,
                    _ => Color::Ivory
                },
                15 => match 1.d3() {
                    ..=1 => Color::Black,
                    2 => Color::Ebony,
                    _ => Color::TrueBlack
                },
                16 => Color::Gray(tint),
                17 => match 1.d3() {
                    ..=1 => Color::Maroon(tint),
                    2 => Color::ReddishBrown(tint),
                    _ => Color::PurplishBrown(tint)
                },
                18 => Color::Silver,
                19 => Color::Gold,
                _  => choice(Some(Tint::random()))
            }
        }

        choice(None)
    }
}
