use dicebag::DiceExt;

use crate::{base_culture::BaseCulture, culture::Culture};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Title {
    Hetman,
    Knight,
    Prince,
    Baronet,
    Baron,
    Count,
    Subchieftain,
    Jarl,
    Viscount,
    Chieftain,
    Marquis,
    Duke,
    Archduke,
    RoyalPrince,
    Kahn,
    King,
    HighKing,
    Emperor,
}

impl Title {
    /**
     Generate a random noble title.

     **Params**
     * *culture* - some [Culture] reference.

     **Returns** a noble [title][Title].
     */
    pub fn random(culture: &Culture) -> Self {
        match culture.base() {
            BaseCulture::Primitive => match 1.d100() {
                ..=1 => Self::HighKing,
                ..=30 => Self::Chieftain,
                _ => Self::Subchieftain
            },

            BaseCulture::Nomad => match 1.d100() {
                ..=10 => Self::Kahn,
                ..=40 => Self::Chieftain,
                ..=80 => Self::Subchieftain,
                _ => Self::Hetman
            },

            BaseCulture::Barbarian => match 1.d100() {
                ..=2 => Self::HighKing,
                ..=15 => Self::King,
                ..=25 => Self::RoyalPrince,
                ..=45 => Self::Chieftain,
                ..=60 => Self::Jarl,
                ..=70 => Self::Subchieftain,
                ..=75 => Self::Baron,
                ..=80 => Self::Prince,
                _ => Self::Hetman
            },

            BaseCulture::Civilized(_) => match 1.d100() {
                ..=1 => Self::Emperor,
                ..=5 => Self::King,
                ..=15 => Self::RoyalPrince,
                ..=20 => Self::Archduke,
                ..=25 => Self::Duke,
                ..=35 => Self::Marquis,
                ..=50 => Self::Viscount,
                ..=60 => Self::Count,
                ..=75 => Self::Baron,
                ..=78 => Self::Baronet,
                ..=90 => Self::Prince,
                _ => Self::Knight
            }
        }
    }

    /**
     Generate (pseudo)random ***TiMod*** based on [Title].

     **Params**
     * *prince_parent_timod* - parents' ***TiMod*** if self is [Title::Prince], otherwise ignored.
     
     **Returns** ***TiMod***.
     */
    pub(crate) fn random_timod(&self, prince_parent_timod: Option<&i32>) -> i32 {
        match self {
            Self::Archduke |
            Self::RoyalPrince => 4.d10(),
            Self::Baron => 2.d10(),
            Self::Baronet => 2.d8(),
            Self::Chieftain |
            Self::Count |
            Self::Jarl => 3.d6(),
            Self::Duke => 4.d8(),
            Self::Emperor => 60,
            Self::Hetman => 1.d6(),
            Self::HighKing => 5.d10(),
            Self::Kahn => 5.d8(),
            Self::King => 39,
            Self::Knight |
            Self::Subchieftain => 2.d6(),
            Self::Marquis => 3.d10(),
            Self::Prince => match prince_parent_timod {
                Some(t) => ((t * 1.d10() * 10) as f64 / 100.0) as i32,
                _ => 4.d10()
            },
            Self::Viscount => 3.d8()
        }
    }

    /**
     Generate `num` random land title(s), if any.

     **Returns** a vec of strings.
     */
    pub(crate) fn random_land_titles(&self) -> Vec<String> {
        // Generate (optional) parts #2 and #3 of a land title.
        fn part2_3() -> Option<(&'static str, &'static str)> {
            // Generate part #3 of a land title.
            fn part3() -> &'static str {
                match 1.d20() {
                    ..=1 => "Coasts",
                    2 => "Creation",
                    3 => "Domain",
                    4 => "Downs",
                    5 => "Fens",
                    6 => "Forests",
                    7 => "Garth",
                    8 => "Heath",
                    9 => "Hills",
                    10 => "Isles",
                    11 => "Marches",
                    12 => "Moors",
                    13 => "Mountains",
                    14 => "Pale",
                    15 => "Reaches",
                    16 => "Shire",
                    17 => "Steppe",
                    18 => "Uplands",
                    19 => "Wastes",
                    _  => "Waves"
                }
            }

            match 1.d20() {
                ..=10 => None,
                11 => Some(("Highland", part3())),
                12 => Some(("Lowland", part3())),
                13 => Some(("Upper", part3())),
                14 => Some(("Lower", part3())),
                15 => Some(("Seaward", part3())),
                16 => Some(("Northern", part3())),
                17 => Some(("Eastern", part3())),
                18 => Some(("Southern", part3())),
                19 => Some(("Western", part3())),
                _  => Some(("Frozen", part3()))
            }
        }

        let mut land_titles = vec![];
        for _ in 0..match self {
            Self::Archduke => 1.d3() + 1,
            Self::Baron    => if 1.d100() < 76 {1} else {0},
            Self::Baronet  => if 1.d100() < 51 {1} else {0},
            Self::Count    => if 1.d100() < 91 {1} else {0},
            Self::Duke     => 1.d3(),
            Self::Emperor  => 1.d4() + 3,
            Self::HighKing |
            Self::Kahn     => 1.d6(),
            Self::King     => 1.d4() + 1,
            Self::Knight   => if 1.d100() < 36 {1} else {0},
            Self::Marquis  => 1.d2(),
            Self::RoyalPrince => 1.d4(),
            Self::Viscount => 1,
            _ => 0
        } {
            let p1 = match 1.d20() {
                ..=1 => "Commander",
                2 => "Custodian",
                3 => "Grim Sentinel",
                4 => "High Champion",
                5 => "Honored Defender",
                6 => "Iron Tower",
                7 => "Lord Protector",
                8 => "Liberator",
                9 => "Lord Governor",
                10 => "Lord Guardian",
                11 => "Keeper",
                12 => "Preserver",
                13 => "Marshall",
                14 => "Ranger",
                15 => "Regent",
                16 => "Retaliator",
                17 => "Swordmaster",
                18 => "Vindicator",
                19 => "Warden",
                _  => "Watchwarder"
            };
            if let Some(p2_3) = part2_3() {
                land_titles.push(format!("{p1} of the {} {}", p2_3.0, p2_3.1))
            } else {
                land_titles.push(p1.to_string())
            }
        }

        land_titles
    }

    /**
     **Returns** self stringified as `&str`.
     */
    pub fn as_str(&self) -> &str {
        match self {
            Self::Archduke => "Archduke",
            Self::Baron => "Baron",
            Self::Baronet => "Baronet",
            Self::Chieftain => "Chieftain",
            Self::Count => "Count (Earl)",
            Self::Duke => "Duke",
            Self::Emperor => "Emperor",
            Self::Hetman => "Hetman",
            Self::HighKing => "High-King",
            Self::Jarl => "Jarl",
            Self::Kahn => "Kahn",
            Self::King => "King",
            Self::Knight => "Knight",
            Self::Marquis => "Marquis",
            Self::Prince => "Prince",
            Self::RoyalPrince => "Royal Prince",
            Self::Subchieftain => "Subchieftain",
            Self::Viscount => "Viscount",
        }
    }
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod title_tests {
    use super::Title;

    #[test]
    fn prince_random_timod_works() {
        let prince_parent_title = Title::Duke;
        let parent_timod = prince_parent_title.random_timod(None);
        let prince = Title::Prince;
        for _ in 0..=10_000 {
            let prince_timod = prince.random_timod(Some(&parent_timod));
            assert!(prince_timod <= parent_timod);
        }
    }
}
