use crate::{society::{NobleTitle, culture::CultureType}, dice::DiceExt};

#[derive(Clone)]
pub struct Nobility {
    title: NobleTitle,
    timod: i32,
    land_titles: Vec<String>,
    land_holdings: i32,
}

struct StrWrap {
    t: &'static str
}

const SPC_TITLE_1: &'static [StrWrap] = &[
    StrWrap {t:"Commander"},
    StrWrap {t:"Custodian"},
    StrWrap {t:"Grim Sentinel"},
    StrWrap {t:"High Champion"},
    StrWrap {t:"Honored Defender"},
    StrWrap {t:"Iron Tower"},
    StrWrap {t:"Lord Protector"},
    StrWrap {t:"Liberator"},
    StrWrap {t:"Lord Governor"},
    StrWrap {t:"Lord Guardian"},
    StrWrap {t:"Keeper"},
    StrWrap {t:"Preserver"},
    StrWrap {t:"Marshall"},
    StrWrap {t:"Ranger"},
    StrWrap {t:"Regent"},
    StrWrap {t:"Retaliator"},
    StrWrap {t:"Swordmaster"},
    StrWrap {t:"Vindicator"},
    StrWrap {t:"Warden"},
    StrWrap {t:"Watchwarder"},
];
const SPC_TITLE_2: &'static [StrWrap] = &[
    StrWrap {t:"Highland"},
    StrWrap {t:"Lowland"},
    StrWrap {t:"Upper"},
    StrWrap {t:"Lower"},
    StrWrap {t:"Seaward"},
    StrWrap {t:"Northern"},
    StrWrap {t:"Eastern"},
    StrWrap {t:"Southern"},
    StrWrap {t:"Western"},
    StrWrap {t:"Frozen"},
    StrWrap {t:"Scorched"},
];
const SPC_TITLE_3: &'static [StrWrap] = &[
    StrWrap {t:"coasts"},
    StrWrap {t:"creation"},
    StrWrap {t:"domain"},
    StrWrap {t:"downs"},
    StrWrap {t:"fens"},
    StrWrap {t:"forests"},
    StrWrap {t:"garth"},
    StrWrap {t:"heath"},
    StrWrap {t:"hills"},
    StrWrap {t:"isles"},
    StrWrap {t:"marches"},
    StrWrap {t:"moors"},
    StrWrap {t:"mountains"},
    StrWrap {t:"pale"},
    StrWrap {t:"reaches"},
    StrWrap {t:"shire"},
    StrWrap {t:"steppe"},
    StrWrap {t:"uplands"},
    StrWrap {t:"wastes"},
    StrWrap {t:"waves"},
];


impl Nobility {
    fn mk_highking()-> Nobility {Nobility { title: NobleTitle::HIGHKING, timod: 5.d(10), land_titles: Nobility::spc_titles(1.d(6)), land_holdings: 1.d(20)*5 }}
    fn mk_king()->     Nobility {Nobility { title: NobleTitle::KING, timod: 39, land_titles: Nobility::spc_titles(1.d(4)+1), land_holdings: 1.d(10)*10 }}
    fn mk_prince_r()-> Nobility {Nobility { title: NobleTitle::ROYALPRINCE, timod: 4.d(10), land_titles: Nobility::spc_titles(1.d(4)), land_holdings: 70.maybe(1.d(20)*5) }}
    fn mk_chief()->    Nobility {Nobility {title: NobleTitle::CHIEF, timod: 3.d(6), land_titles: Vec::new(), land_holdings: 40.maybe(2.d(6)+8) }}
    fn mk_subchief()-> Nobility {Nobility { title: NobleTitle::SUBCHIEF, timod: 2.d(6), land_titles: Vec::new(), land_holdings: 30.maybe(1.d(8)) }}
    fn mk_baron()->    Nobility {Nobility { title: NobleTitle::BARON, timod: 2.d(10), land_titles: Nobility::spc_titles(75.maybe(1)), land_holdings: 60.maybe(1.d(10)+4) }}
    fn mk_hetman()->   Nobility {Nobility { title: NobleTitle::HETMAN, timod: 1.d(6), land_titles: Vec::new(), land_holdings: 85.maybe(1.d(4)) }}
    fn mk_archduke()-> Nobility {Nobility { title: NobleTitle::ARCHDUKE, timod: 4.d(10), land_titles: Self::spc_titles(1.d(3)+1), land_holdings: 75.maybe(1.d(10)*5) }}
    fn mk_prince(parent:Nobility)-> Nobility {
        if 1.d(100) < 21 {
            let p = Nobility::mk_archduke();
            Nobility {title: NobleTitle::ROYALPRINCE, timod: p.timod, land_holdings: p.land_holdings, land_titles: p.land_titles}
        } else {
            let d = (1.d(10)*10) as f64 * 0.01;
            Nobility {title: NobleTitle::PRINCE, timod: (parent.timod as f64 * d) as i32, land_titles: Vec::new(), land_holdings: 0}
        }
    }

    pub fn title(&self)-> NobleTitle {self.title}
    pub fn timod(&self)-> i32 {self.timod}
    pub fn land_titles(&self)-> &Vec<String> {&self.land_titles}
    pub fn land_holdings(&self)-> i32 {self.land_holdings}
    pub fn new(culture_type:CultureType)-> Nobility {
        match culture_type {
            CultureType::PRIMITIVE => match 1.d(100) {
                1 => Self::mk_highking(),
                x if x < 31 => Self::mk_subchief(),
                _ => Self::mk_subchief()
            },
            CultureType::NOMAD => match 1.d(100) {
                x if x < 11 => Nobility { title: NobleTitle::KAHN, timod: 5.d(8), land_titles: Self::spc_titles(1.d(6)), land_holdings: 30.maybe(1.d(10)*5) },
                x if x < 41 => Self::mk_chief(),
                x if x < 81 => Self::mk_subchief(),
                _ => Self::mk_hetman()
            },
            CultureType::BARBARIAN => match 1.d(100) {
                1|2 => Self::mk_highking(),
                x if x <= 15 => Self::mk_king(),
                x if x <= 25 => Self::mk_prince_r(),
                x if x <= 45 => Self::mk_chief(),
                x if x <= 60 => Nobility { title: NobleTitle::JARL, timod: 3.d(6), land_titles: Vec::new(), land_holdings: 70.maybe(1.d(6)+4) },
                x if x <= 70 => Self::mk_subchief(),
                x if x <= 75 => Self::mk_baron(),
                x if x <= 80 => Self::mk_prince(Nobility::new(culture_type)),
                _ => Self::mk_hetman()
            },
            _ => match 1.d(100) {
                1 => Nobility { title: NobleTitle::EMPEROR, timod: 60, land_titles: Self::spc_titles(1.d(4)+3), land_holdings: 1.d(20)*10 },
                x if x <= 5 => Self::mk_king(),
                x if x <= 15 => Self::mk_prince_r(),
                x if x <= 20 => Self::mk_archduke(),
                x if x <= 25 => Nobility { title: NobleTitle::DUKE, timod: 4.d(8), land_titles: Self::spc_titles(1.d(3)), land_holdings: 85.maybe(1.d(10)*5) },
                x if x <= 35 => Nobility { title: NobleTitle::MARQUIS, timod: 3.d(10), land_titles: Self::spc_titles(1.d(2)), land_holdings: 60.maybe(1.d(20)+12) },
                x if x <= 50 => Nobility { title: NobleTitle::VISCOUNT, timod: 3.d(8), land_titles: Self::spc_titles(1), land_holdings: 50.maybe(1.d(20)+10) },
                x if x <= 60 => Nobility { title: NobleTitle::COUNT, timod: 3.d(6), land_titles: Self::spc_titles(90.maybe(1)), land_holdings: 40.maybe(1.d(20)+4) },
                x if x <= 75 => Self::mk_baron(),
                76 | 77 | 78 => Nobility { title: NobleTitle::BARONET, timod: 2.d(8), land_titles: Self::spc_titles(50.maybe(1)), land_holdings: 30.maybe(1.d(10)) },
                x if x <= 90 => Self::mk_prince(Nobility::new(culture_type)),
                _ => Nobility { title: NobleTitle::KNIGHT, timod: 2.d(6), land_titles: Self::spc_titles(35.maybe(1)), land_holdings: 60.maybe(1.d(4)) }
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
        let t1 = &SPC_TITLE_1[(1.d(SPC_TITLE_1.len())-1)as usize];
        let r = 1.d(20) as usize;
        if r < SPC_TITLE_2.len() {
            t1.t.to_owned() +" of the "+ SPC_TITLE_2[r-1].t +" "+ SPC_TITLE_3[(1.d(20)-1)as usize].t
        } else {t1.t.to_string()}
    }
}
