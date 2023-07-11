use lazy_st::lazy;

use crate::{society::{NobleTitle, culture::CultureType}, dice::DiceExt};

pub struct Nobility {
    title: NobleTitle,
    timod: i32,
    land_titles: Vec<String>,
    land_holdings: i32,
}

const SPC_TITLE_1:Vec<&'static str> = vec![
    "Commander", "Custodian", "Grim Sentinel", "High Champion", "Honored Defender",
    "Iron Tower", "Lord Protector", "Liberator", "Lord Governor", "Lord Guardian",
    "Keeper", "Preserver", "Marshall", "Ranger", "Regent",
    "Retaliator", "Swordmaster", "Vindicator", "Warden", "Watchwarder",
];
const SPC_TITLE_2:Vec<&'static str> = vec![
    "Highland", "Lowland", "Upper", "Lower", "Seaward",
    "Northern", "Eastern", "Southern", "Western", "Frozen",
    "Scorched",
];
const SPC_TITLE_3:Vec<&'static str> = vec![
    "coasts", "creation", "domain", "downs", "fens", "forests", "garth", "heath", "hills", "isles",
    "marches", "moors", "mountains", "pale", "reaches", "shire", "steppe", "uplands", "wastes", "waves",
];

impl Nobility {
    fn mk_emperor() -> Nobility {Nobility {title: NobleTitle::EMPEROR, timod: 60, land_holdings: 1.d(20)*10, land_titles: Nobility::spc_titles(1.d(4)+3)}}
    fn mk_highking() -> Nobility {Nobility { title: NobleTitle::HIGHKING, timod: 5.d(10), land_titles: Nobility::spc_titles(1.d(6)), land_holdings: 1.d(20)*5 }}
    fn mk_chief() -> Nobility {Nobility {title: NobleTitle::CHIEF, timod: 3.d(6), land_titles: Vec::new(), land_holdings: 40.maybe(2.d(6)+8) }}
    fn mk_subchief() -> Nobility {Nobility { title: NobleTitle::SUBCHIEF, timod: 2.d(6), land_titles: Vec::new(), land_holdings: 30.maybe(1.d(8)) }}
    fn mk_hetman() -> Nobility {Nobility { title: NobleTitle::HETMAN, timod: 1.d(6), land_titles: Vec::new(), land_holdings: 85.maybe(1.d(4)) }}

    pub fn title(&self) -> NobleTitle {self.title}
    pub fn timod(&self) -> i32 {self.timod}
    pub fn land_titles(&self) -> &Vec<String> {&self.land_titles}
    pub fn land_holdings(&self) -> i32 {self.land_holdings}
    pub fn new(culture_type:CultureType) -> Nobility {
        match culture_type {
            CultureType::PRIMITIVE => match 1.d(100) {
                1 => Self::mk_highking(),
                x if x < 31 => Self::mk_subchief(),
                _ => Self::mk_subchief()
            },
            CultureType::NOMAD => match 1.d(100) {
                x if x < 11 => Nobility { title: NobleTitle::KAHN, timod: 5.d(8), land_titles: Nobility::spc_titles(1.d(6)), land_holdings: 30.maybe(1.d(10)*5) },
                x if x < 41 => Self::mk_chief(),
                x if x < 81 => Self::mk_subchief(),
                _ => Self::mk_hetman()
            },
            CultureType::BARBARIAN => match 1.d(100) {
                1|2 => Self::mk_highking(),
                x if x <= 15 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                x if x <= 25 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                x if x <= 45 => Self::mk_chief(),
                x if x <= 60 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                x if x <= 70 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                x if x <= 75 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                x if x <= 80 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                _ => Nobility { title: (), timod: (), land_titles: (), land_holdings: () }
            },
            _ => match 1.d(100) {
                1 => Nobility { title: NobleTitle::EMPEROR, timod: 60, land_titles: (), land_holdings: 1.d(20)*10 },
                x if x <= 5 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                x if x <= 15 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                x if x <= 20 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                x if x <= 25 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                x if x <= 35 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                x if x <= 50 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                x if x <= 60 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                x if x <= 75 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                76 | 77 | 78 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                x if x <= 90 => Nobility { title: (), timod: (), land_titles: (), land_holdings: () },
                _ => Nobility { title: (), timod: (), land_titles: (), land_holdings: () }
            }
        }
    }

    /// Generate a list of special titles (for [nobility][Nobility]).
    /// 
    /// ### Arguments
    /// * `num` - Number of titles to generate.
    /// 
    fn spc_titles(num:i32) -> Vec<String> {
        let mut v = Vec::new();
        for _ in 0..num {
            v.push(Self::spc_title());
        }
        v
    }

    /// Generate a special title (for [nobility][Nobility]).
    fn spc_title() -> String {
        let t1 = SPC_TITLE_1[(1.d(SPC_TITLE_1.len())-1)as usize];
        let r = 1.d(20) as usize;
        if r < SPC_TITLE_2.len() {
            t1.to_owned()+" of the "+SPC_TITLE_2[r-1]+" "+SPC_TITLE_3[(1.d(20)-1)as usize]
        } else {t1.to_string()}
    }
}
