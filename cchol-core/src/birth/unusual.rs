use dicebag::DiceExt;

/// Some unusual birth circumstance containers.
pub enum UnusualBirth {
    None,
    OneOccuarance(UnusualBirthOccurance),
    Multiple { occurances: Vec<UnusualBirthOccurance>, gm_select_occurances: Option<Vec<UnusualBirthOccurance>> },
}

impl UnusualBirth {
    /// Generate random birth circumstance(s), if any.
    pub fn random(bimod: i32) -> Option<Self> {
        match 1.d100() + bimod {
            ..=60 => None,
            ..=76 => Some(Self::OneOccuarance(UnusualBirthOccurance::random())),
            ..=85 => Some(Self::Multiple { occurances: vec![UnusualBirthOccurance::random(), UnusualBirthOccurance::random()], gm_select_occurances: None }),
            ..=92 => Some(Self::Multiple { occurances: vec![UnusualBirthOccurance::random()], gm_select_occurances: Some(vec![UnusualBirthOccurance::random()]) }),
            ..=94 => Some(Self::Multiple { occurances: vec![
                UnusualBirthOccurance::random(),
                UnusualBirthOccurance::random(),
                UnusualBirthOccurance::random()], gm_select_occurances: None }),

            ..=97 => Some({
                let mut occurances = vec![];
                let mut gm_select = vec![];
                let gms = 1.d2();
                for _ in 0..3-gms {
                    occurances.push(UnusualBirthOccurance::random())
                };
                for _ in 0..gms {
                    gm_select.push(UnusualBirthOccurance::random())
                };
                Self::Multiple { occurances, gm_select_occurances: gm_select.into() }
            }),

            98 => Some(Self::Multiple { occurances: {
                let mut occurances = vec![];
                for _ in 0..4 {
                    occurances.push(UnusualBirthOccurance::random())
                }
                occurances
            }, gm_select_occurances: None }),

            _ => Some({
                let mut occurances = vec![];
                let mut gm_select = vec![];
                let gms = 1.d3();
                for _ in 0..4-gms {
                    occurances.push(UnusualBirthOccurance::random())
                };
                for _ in 0..gms {
                    gm_select.push(UnusualBirthOccurance::random())
                };
                Self::Multiple { occurances, gm_select_occurances: gm_select.into() }
            })
        }
    }
}

pub enum UBO39_41 {
    O1 { mag: i32 },
    O2_3,
    O4_5,
    O6 { mag: i32 },
    O7,
    O8_9,
    O10
}

pub enum UBO42_44 {
    O1 { mag: i32 },
    O2_3,
    O4_5,
    O6 { mag: i32 },
    O7,
    O8_9,
    O10
}

pub enum UnusualBirthOccurance {
    O01_05,
    O06_10,
    O11_20,
    O21_23,
    O24_25,
    O26_27,
    O28_31 { separated_at_birth: bool, drastically_different: bool },
    O32_34,
    O35_37,
    O38,
    O39_41(UBO39_41),
    O42_44(UBO42_44),
    O45,//TODO: #45 is missing from the book -create something.
    O46_48,//TODO: family curse 868
    O49_50,//TODO: golden goose egg...
    O51_53,
    O54_55,
    O56,
    O57,
    O58_62,//TODO: change family state to 'adopted'
    O63_64,//TODO: tragedy 528
    O65_69,//TODO: birthmark 866
    O70_75,//TODO: curse 868
    O76_81,//TODO: blessing 869
    O82_85,//TODO: twin
    O86,
    O87_88,//TODO: death situation 545,
    O89_93,//TODO: phys.affl. 874
    O94,   //TODO: psi powers 873
    O95_99,//TODO: gifts 863
    O100,  //---ROLL 2x
    O101_105,
    O106_110,//TODO: attribute alterations, 874, 868, 648
    O111_120,//TODO: attribute alterations, 874, 863, 869, determine Deity
}
