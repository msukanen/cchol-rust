use std::collections::HashSet;

use dicebag::DiceExt;
use rpga_generic::gender::Gender;

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

#[derive(PartialEq, Eq, Hash)]
enum UBO39_44H { O1, O2, O4, O6, O7, O8, O10 }

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
    O39_41(Vec<UBO39_41>),
    O42_44(Vec<UBO42_44>),
    O45,//TODO: #45 is missing from the book -create something.
    O46_48,//TODO: family curse 868
    O49_50 { ch_has_it: bool, magical: bool },
    O51_53,
    O54_55,
    O56,
    O57,
    O58_62,//TODO: change family state to 'adopted'
    O63_64,//TODO: tragedy 528
    O65_69,//TODO: birthmark 866
    O70_75,//TODO: curse 868
    O76_81,//TODO: blessing 869
    O82_85(Gender),
    O86,
    O87_88,//TODO: death situation 545,
    O89_93,//TODO: phys.affl. 874
    O94,   //TODO: psi powers 873
    O95_99,//TODO: gifts 863
    O100(Box<UnusualBirthOccurance>, Box<UnusualBirthOccurance>),
    O101_105,
    O106_110,//TODO: attribute alterations, 874, 868, 648
    O111_120,//TODO: attribute alterations, 874, 863, 869, determine Deity
}

impl UnusualBirthOccurance {
    pub fn random() -> Self {
        fn choice(add: i32) -> UnusualBirthOccurance {
            fn ch_ubo(ubo: &HashSet<UBO39_44H>) -> UBO39_44H {
                let v = match 1.d10() {
                    ..=1 => UBO39_44H::O1,
                    ..=3 => UBO39_44H::O2,
                    ..=5 => UBO39_44H::O4,
                    6    => UBO39_44H::O6,
                    7    => UBO39_44H::O7,
                    ..=9 => UBO39_44H::O8,
                    _    => UBO39_44H::O10
                };
                
                if ubo.contains(&v) { ch_ubo(ubo) } else { v }
            }

            match 1.d100() + add {
                ..=5 => UnusualBirthOccurance::O01_05,
                ..=10 => UnusualBirthOccurance::O06_10,
                ..=20 => UnusualBirthOccurance::O11_20,
                ..=23 => UnusualBirthOccurance::O21_23,
                ..=25 => UnusualBirthOccurance::O24_25,
                ..=27 => UnusualBirthOccurance::O26_27,
                ..=31 => UnusualBirthOccurance::O28_31 { separated_at_birth: 1.d100() < 21, drastically_different: 1.d6() == 6 },
                ..=34 => UnusualBirthOccurance::O32_34,
                ..=37 => UnusualBirthOccurance::O35_37,
                38    => UnusualBirthOccurance::O38,
                ..=41 => {
                    let mut ubo = HashSet::new();
                    for _ in 0..1.d3() {
                        ubo.insert(ch_ubo(&ubo));
                    }
                    let mut ubos = vec![];
                    for x in ubo {
                        match x {
                            UBO39_44H::O1 => ubos.push(UBO39_41::O1 { mag: 1.d6() }),
                            UBO39_44H::O2 => ubos.push(UBO39_41::O2_3),
                            UBO39_44H::O4 => ubos.push(UBO39_41::O4_5),
                            UBO39_44H::O6 => ubos.push(UBO39_41::O6 { mag: -1.d6() }),
                            UBO39_44H::O7 => ubos.push(UBO39_41::O7),
                            UBO39_44H::O8 => ubos.push(UBO39_41::O8_9),
                            UBO39_44H::O10 => ubos.push(UBO39_41::O10),
                        }
                    }
                    UnusualBirthOccurance::O39_41(ubos)
                },
                ..=44 => {
                    let mut ubo = HashSet::new();
                    for _ in 0..1.d3() {
                        ubo.insert(ch_ubo(&ubo));
                    }
                    let mut ubos = vec![];
                    for x in ubo {
                        match x {
                            UBO39_44H::O1 => ubos.push(UBO42_44::O1 { mag: 1.d6() }),
                            UBO39_44H::O2 => ubos.push(UBO42_44::O2_3),
                            UBO39_44H::O4 => ubos.push(UBO42_44::O4_5),
                            UBO39_44H::O6 => ubos.push(UBO42_44::O6 { mag: -1.d6() }),
                            UBO39_44H::O7 => ubos.push(UBO42_44::O7),
                            UBO39_44H::O8 => ubos.push(UBO42_44::O8_9),
                            UBO39_44H::O10 => ubos.push(UBO42_44::O10),
                        }
                    }
                    UnusualBirthOccurance::O42_44(ubos)
                },
                45 => todo!("#45 is not implemented in the book?! Figure out something here..."),
                ..=48 => todo!("UnusualBirthOccurance::O46_48"),
                ..=50 => {
                    let r = 1.d10();
                    UnusualBirthOccurance::O49_50 { ch_has_it: r > 6, magical: r > 9 }
                },
                ..=53 => UnusualBirthOccurance::O51_53,
                ..=55 => UnusualBirthOccurance::O54_55,
                56    => UnusualBirthOccurance::O56,
                57    => UnusualBirthOccurance::O57,
                ..=62 => UnusualBirthOccurance::O58_62,
                ..=64 => todo!("UnusualBirthOccurance::O63_64"),
                ..=69 => todo!("UnusualBirthOccurance::O65_69"),
                ..=75 => todo!("UnusualBirthOccurance::O70_75"),
                ..=81 => todo!("UnusualBirthOccurance::O76_81"),
                ..=85 => UnusualBirthOccurance::O82_85(Gender::random()),
                86    => UnusualBirthOccurance::O86,
                ..=88 => todo!("UnusualBirthOccurance::O87_88"),
                ..=93 => todo!("UnusualBirthOccurance::O89_93"),
                94    => todo!("UnusualBirthOccurance::O94"),
                ..=99 => todo!("UnusualBirthOccurance::O95_99"),
                100   => UnusualBirthOccurance::O100(Box::new(choice(add)), Box::new(choice(add))),
                ..=110 => todo!("UnusualBirthOccurance::O101_105"),
                _      => todo!("UnusualBirthOccurance::O111_120")
            }
        }

        choice(0)
    }
}
