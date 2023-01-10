use crate::{dice::DiceExt, attribute::Attribute, society::culture::CultureType};

pub mod environment;

pub trait Skill {
    fn name(&self) -> &str;
    fn rank(&self) -> i32;
}

pub struct BasicSkill {
    name: String,
    rank: i32,
}

impl Skill for BasicSkill {
    fn name(&self) -> &str { &self.name }
    fn rank(&self) -> i32 { self.rank }
}

impl BasicSkill {
    pub fn new(name:String, rank:i32) -> Box<dyn Skill> { Box::new(BasicSkill{name, rank})}
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DegreeOfInterest {
    CASUAL,
    SPORADIC,
    DEVOTED,
    CONSUMINGPASSION,
}

impl DegreeOfInterest {
    pub fn new() -> DegreeOfInterest {
        match 1.d(10) {
            1|2 => Self::CASUAL,
            8|9 => Self::DEVOTED,
            10 => Self::CONSUMINGPASSION,
            _ => Self::SPORADIC
        }
    }
}

pub struct Hobby {
    name: String,
    rank: i32,
    doi: DegreeOfInterest,
    extra_skill: Option<Box<dyn Skill>>,
    affected_attribute: Option<Box<dyn Attribute>>,
}

impl Hobby {
    pub fn new(child:bool, cultureType:&CultureType, s:i32) -> Box<dyn Skill> {
        let mut rank = 1.d(4);
        if child { rank -= 2 }
        rank += match cultureType {
            CultureType::PRIMITIVE => -2,
            CultureType::NOMAD => -1,
            CultureType::CIVILIZED
            | CultureType::DECADENT => 1,
            _ => 0
        };
        let doi = DegreeOfInterest::new();
        let extra_skill = None;
        let affected_attribute = None;
        Box::new(
        match 1.d(20) {
            1 => Hobby{name:"collect something".to_string(), rank, doi, extra_skill, affected_attribute},
            2 => Hobby{name:"dancing".to_string(), rank, doi, extra_skill, affected_attribute},
            3 => Hobby{name:"play instrument".to_string(), rank, doi, extra_skill, affected_attribute},
            4 => Hobby{name:"reading for enjoyment".to_string(), rank, doi, extra_skill, affected_attribute},
            5 => Hobby{name:"writing creatively".to_string(), rank, doi, extra_skill, affected_attribute},
            6 => Hobby{name:"dramatics (acting)".to_string(), rank, doi, extra_skill, affected_attribute},
            7 => Hobby{
                name:match 1.d(3) {
                    1 => "drawing",
                    2 => "painting",
                    _ => "sculpting"
                }.to_string(), rank, doi, extra_skill, affected_attribute},
            8 => Hobby{name:"needlework".to_string(), rank, doi, extra_skill, affected_attribute},
            9 => Hobby{name:"sing".to_string(), rank, doi, extra_skill, affected_attribute},
            10 => Hobby{
                name:("study ".to_string() + match 1.d(8) {
                    1 => "history",
                    2 => "religion",
                    3 => "art",
                    4 => "astronomy",
                    5 => "astrology",
                    6 => "other cultures",
                    7 => "magic",
                    _ => "weapons"
                }).to_string(), rank, doi, extra_skill, affected_attribute},
            11 => Hobby{name:"dancing".to_string(), rank, doi, extra_skill, affected_attribute},
            12 => Hobby{name:"building models".to_string(), rank, doi, extra_skill, affected_attribute},
            13 => Hobby{name:"appreciate arts".to_string(), rank, doi, extra_skill, affected_attribute},
            14 => Hobby{name:"hairdressing and cosmetics".to_string(), rank, doi, extra_skill, affected_attribute},
            15 => Hobby{name:"sports hunting".to_string(), rank, doi, extra_skill, affected_attribute},
            16 => Hobby{name:"gardening".to_string(), rank, doi, extra_skill, affected_attribute},
            17 => Hobby{name:"breeding dogs".to_string(), rank, doi, extra_skill, affected_attribute},
            18 => Hobby{name:"animal husbandry".to_string(), rank, doi, extra_skill, affected_attribute},
            19 => Hobby{name:"sports fishing".to_string(), rank, doi, extra_skill, affected_attribute},
            _ => Hobby{name:"heraldry".to_string(), rank, doi, extra_skill, affected_attribute}
        })
    }

    pub fn degree_of_interest(&self) -> DegreeOfInterest { self.doi }
    pub fn extra_skill(&self) -> Option<&Box<dyn Skill + 'static>> { self.extra_skill.as_ref() }
    pub fn affected_attribute(&self) -> Option<&Box<dyn Attribute + 'static>> { self.affected_attribute.as_ref() }
}

impl Skill for Hobby {
    fn name(&self) -> &str { &self.name }
    fn rank(&self) -> i32 { self.rank }
}
