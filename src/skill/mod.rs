use crate::{dice::DiceExt, attribute::{Attribute, Strength, Constitution, Dexterity}, society::culture::CultureType};

pub mod environment;

/// Container for literacy chances.
pub struct LiteracyChance {
    /// Chance to be literate in native language.
    percentage: i32,
    /// Is the chance 'flat' or additive with other sources.
    flat: bool,
    /// Chance to be literate in other language(s).
    percentage_other: i32,
    /// Is the chance 'flat' or additive with other sources.
    flat_other: bool,
}
impl LiteracyChance {
    /// Return the chance to be literate in native language.
    /// 
    /// **Return:** `(chance%, chance-flatness)`
    pub fn percentage(&self) -> (i32, bool) {(self.percentage, self.flat)}
    /// Return the chance to be literate in other language(s).
    /// 
    /// **Return:** `(applicable, chance%, chance-flatness)`
    pub fn percentage_other(&self) -> (bool, i32, bool) {(self.percentage_other>0, self.percentage_other, self.flat_other)}
    /// Generate a new *native* language chance.
    pub fn new_single(percentage:i32, flat:bool) -> LiteracyChance { LiteracyChance { percentage, flat, percentage_other: 0, flat_other: true }}
    /// Generate a new *native* and *other* language(s) chance.
    pub fn new_dual(percentage:i32, flat:bool, percentage2:i32, flat2:bool) -> LiteracyChance { LiteracyChance { percentage, flat, percentage_other: percentage2, flat_other: flat2 }}
}

/// Interface for all sorts of skills and skill-like abilities.
pub trait Skill {
    /// Return name of the skill.
    fn name(&self) -> &str;
    /// Return rank of the skill.
    fn rank(&self) -> i32;
}

/// A pure, basic, skill of some sort.
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
/// Degrees of [hobby][Hobby] interest.
pub enum DegreeOfInterest {
    CASUAL,
    SPORADIC,
    DEVOTED,
    CONSUMINGPASSION,
}

impl DegreeOfInterest {
    /// Return a random [degree of interest][DegreeOfInterest].
    pub fn new() -> DegreeOfInterest {
        match 1.d(10) {
            1|2 => Self::CASUAL,
            8|9 => Self::DEVOTED,
            10 => Self::CONSUMINGPASSION,
            _ => Self::SPORADIC
        }
    }
}

/// Hobbies are somewhat specialized [skills][Skill].
pub struct Hobby {
    name: String,
    rank: i32,
    doi: DegreeOfInterest,
    extra_skill: Option<Box<dyn Skill>>,
    affected_attribute: Option<Box<dyn Attribute>>,
}

impl Hobby {
    /// Return a new hobby.
    /// 
    /// ### Arguments
    /// * `child` - the hobby is for a child?
    /// * `culture_type` - [culture][CultureType] that might affect the hobby.
    /// * `society` - [society][Society] that might affect the hobby.
    pub fn new(child:bool, culture_type:CultureType, society:i32) -> Box<dyn Skill> {
        let mut rank = 1.d(4);
        if child { rank -= 2 }
        rank += match culture_type {
            CultureType::PRIMITIVE => -2,
            CultureType::NOMAD => -1,
            CultureType::CIVILIZED
            | CultureType::DECADENT => 1,
            _ => 0
        };
        let doi = DegreeOfInterest::new();
        let mut extra_skill = None;
        let mut affected_attribute = None;
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
            11 => {
                let r = 1.d(8);
                let name = match r {
                    1 => "wrestling",
                    2 => "running",
                    3 => "fencing",
                    4 => "team ball sport",
                    5 => "horse racing",
                    6 => "swimming",
                    7 => "archery",
                    _ => "boxing"
                };
                match r {
                    1 => {affected_attribute = Some(Box::new(Strength::new(1)))},
                    2 => {affected_attribute = Some(Box::new(Constitution::new(1)))},
                    3 => {extra_skill = Some(BasicSkill::new("fencing".to_string(), rank))},
                    4 => {affected_attribute = Some(Box::new(Dexterity::new(1)))},
                    5 => {extra_skill = Some(BasicSkill::new("riding".to_string(), rank))},
                    6 => {extra_skill = Some(BasicSkill::new("swimming".to_string(), rank))},
                    7 => {extra_skill = Some(BasicSkill::new("bow".to_string(), rank))},
                    _ => {extra_skill = Some(BasicSkill::new("boxing".to_string(), rank))}
                };
                Hobby{name: name.to_string(), rank, doi, extra_skill, affected_attribute}
            },
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

    /// Return current [degree of interest][DegreeOfInterest].
    pub fn degree_of_interest(&self) -> DegreeOfInterest { self.doi }
    /// Return (optional) extra [skill][Skill] associated with the hobby.
    pub fn extra_skill(&self) -> Option<&Box<dyn Skill + 'static>> { self.extra_skill.as_ref() }
    /// Return (optional) [attribute][Attribute] affected by the hobby.
    pub fn affected_attribute(&self) -> Option<&Box<dyn Attribute + 'static>> { self.affected_attribute.as_ref() }
}

impl Skill for Hobby {
    fn name(&self) -> &str { &self.name }
    fn rank(&self) -> i32 { self.rank }
}
