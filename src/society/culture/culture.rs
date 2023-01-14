use crate::{skill::{environment::EnvironmentType, BasicSkill, Skill, Hobby, LiteracyChance}, race::Race, dice::DiceExt};

use super::CultureType;

/// Interface for all sorts of cultures.
pub trait Culture {
    /// Returns culture [type][CultureType]/level.
    fn level(&self) -> CultureType;
    /// Returns cultural modifier, **CuMod**.
    fn cumod(&self) -> i32;
    /// Returns native [environment][EnvironmentType] type.
    fn native_env(&self) -> EnvironmentType;
    /// Returns survival skill rank bonus.
    /// 
    /// ### Arguments
    /// * `env` - Requested [environment][EnvironmentType].
    /// 
    fn survival_rank(&self, env:EnvironmentType) -> i32;
    /// Returns basic skills, if any.
    fn basic_skills(&self) -> &[Box<dyn Skill>];
    /// Returns literacy chance(s), if any.
    fn literacy_chance(&self) -> &LiteracyChance;
}

/// Culture factory.
pub struct CultureFactory;
impl CultureFactory {
    /// Generate a random [culture][Culture].
    fn new_any() -> Box<dyn Culture> {
        match 1.d(10) {
            1 => Primitive::new(),
            2|3 => Nomad::new(),
            4|5|6 => Barbarian::new(),
            7|8|9 => Civilized::new(),
            _ => Decadent::new()
        }
    }

    /// Generate a random [culture][Culture].
    /// 
    /// ### Arguments
    /// * `race` - [Race] which may (or may not) impose restrictions on the resulting [culture][Culture].
    /// 
    pub fn new(race:&dyn Race) -> Box<dyn Culture> {
        let mut c: Box<dyn Culture>;
        loop {
            c = CultureFactory::new_any();
            if c.level() <= race.max_culture() {break;}
        } c
    }
}

/// Primitives are a relatively [primitive][Culture].
struct Primitive {
    v_basic_skills: Vec<Box<dyn Skill>>,
    literacy_chance: LiteracyChance,
}
impl Culture for Primitive {
    fn level(&self) -> CultureType { CultureType::PRIMITIVE }
    fn cumod(&self) -> i32 { -3 }
    fn native_env(&self) -> EnvironmentType { EnvironmentType::WILDERNESS }
    fn survival_rank(&self, env:EnvironmentType) -> i32 {
        match env {
            EnvironmentType::WILDERNESS => 5,
            _ => 1
        }
    }
    fn basic_skills(&self) -> &[Box<dyn Skill>] { self.v_basic_skills.as_slice()}
    fn literacy_chance(&self) -> &LiteracyChance { &self.literacy_chance }
}
impl Primitive {
    pub fn new() -> Box<dyn Culture> {Box::new(Primitive{
        v_basic_skills: {
            let mut v = Vec::new();
            v.push(BasicSkill::new("survival".to_string(), 4));
            v.push(BasicSkill::new(match 1.d(3) {
                1 => "club",
                2 => "spear",
                _ => "bow"
            }.to_string(), 3));
            v
        },
        literacy_chance: LiteracyChance::new_single(5, true)
    })}
}

/// Nomad are no-mad, just roaming around [culturally][Culture].
struct Nomad {
    v_basic_skills: Vec<Box<dyn Skill>>,
    literacy_chance: LiteracyChance,
}
impl Culture for Nomad {
    fn level(&self) -> CultureType { CultureType::NOMAD }
    fn cumod(&self) -> i32 { 0 }
    fn native_env(&self) -> EnvironmentType { EnvironmentType::WILDERNESS }
    fn survival_rank(&self, env:EnvironmentType) -> i32 {
        match env {
            EnvironmentType::WILDERNESS => 4,
            _ => 1
        }
    }
    fn basic_skills(&self) -> &[Box<dyn Skill>] { self.v_basic_skills.as_slice() }
    fn literacy_chance(&self) -> &LiteracyChance { &self.literacy_chance }
}
impl Nomad {
    pub fn new() -> Box<dyn Culture> {
        // 20% chance for native, 10% for other lang.
        let l1 = 1.d(100) <= 20;
        let l2 = if l1 { 1.d(100) <= 10 } else { false };
        let literacy_chance = if l2 {
            LiteracyChance::new_dual(100, false, 10, false)
        } else {
            LiteracyChance::new_single(20, false)
        };
        Box::new(Nomad{
            v_basic_skills: {
                let mut v = Vec::new();
                v.push(BasicSkill::new("riding".to_string(), 4));
                v.push(BasicSkill::new("nomad weapon".to_string(), 3));
                v
            },
            literacy_chance
        })
    }
}

/// Barbaric [cultures][Culture] tend to be highly dynamic.
struct Barbarian {
    nat_env: EnvironmentType,
    v_basic_skills: Vec<Box<dyn Skill>>,
    literacy_chance: LiteracyChance,
}
impl Culture for Barbarian {
    fn level(&self) -> CultureType { CultureType::BARBARIAN }
    fn cumod(&self) -> i32 { 2 }
    fn native_env(&self) -> EnvironmentType { self.nat_env }
    fn survival_rank(&self, env:EnvironmentType) -> i32 {
        match env {
            x if self.nat_env == x => 5,
            _ => 1
        }
    }
    fn basic_skills(&self) -> &[Box<dyn Skill>] { self.v_basic_skills.as_slice() }
    fn literacy_chance(&self) -> &LiteracyChance { &self.literacy_chance }
}
impl Barbarian {
    pub fn new() -> Box<dyn Culture> {
        Box::new(Barbarian{
            nat_env: if 1.d(10)<6 {EnvironmentType::WILDERNESS} else {EnvironmentType::URBAN},
            v_basic_skills: {
                let mut v = Vec::new();
                v.push(BasicSkill::new("hand weapon".to_string(), 3));
                v.push(BasicSkill::new("missile weapon".to_string(), 3));
                v
            },
            literacy_chance: LiteracyChance::new_single(10, false)
        })
    }
}

/// Civilized [cultures][Culture] tend to think that they're civilized.
struct Civilized {
    nat_env: EnvironmentType,
    v_basic_skills: Vec<Box<dyn Skill>>,
    literacy_chance: LiteracyChance,
}
impl Culture for Civilized {
    fn level(&self) -> CultureType { CultureType::CIVILIZED }
    fn cumod(&self) -> i32 { 4 }
    fn native_env(&self) -> EnvironmentType { self.nat_env }
    fn survival_rank(&self, env:EnvironmentType) -> i32 {
        match env {
            x if self.nat_env == x => 2,
            _ => 0
        }
    }
    fn basic_skills(&self) -> &[Box<dyn Skill>] { self.v_basic_skills.as_slice() }
    fn literacy_chance(&self) -> &LiteracyChance { &self.literacy_chance }
}
impl Civilized {
    pub fn new() -> Box<dyn Culture> {
        Box::new(Civilized{
            nat_env: if 1.d(10)<4 {EnvironmentType::WILDERNESS} else {EnvironmentType::URBAN},
            v_basic_skills: {
                let mut v = Vec::new();
                v.push(Hobby::new(false, CultureType::CIVILIZED, 2));
                v
            },
            literacy_chance: LiteracyChance::new_single(30, false)
        })
    }
}

/// Decadence, the pinnacle (or bottom of the barrel) of [cultural][Culture] development.
struct Decadent {
    v_basic_skills: Vec<Box<dyn Skill>>,
    literacy_chance: LiteracyChance,
}
impl Culture for Decadent {
    fn level(&self) -> CultureType { CultureType::DECADENT }
    fn cumod(&self) -> i32 { 7 }
    fn native_env(&self) -> EnvironmentType { EnvironmentType::URBAN }
    fn survival_rank(&self, env:EnvironmentType) -> i32 {
        match env {
            EnvironmentType::URBAN => 3,
            _ => 1
        }
    }
    fn basic_skills(&self) -> &[Box<dyn Skill>] { self.v_basic_skills.as_slice() }
    fn literacy_chance(&self) -> &LiteracyChance { &self.literacy_chance }
}
impl Decadent {
    pub fn new() -> Box<dyn Culture> {
        // 20% for native, 10% for other lang.
        let l1 = 1.d(100) <= 20;
        let literacy_chance = if l1 {
            LiteracyChance::new_dual(100, false, 10, false)
        } else {
            LiteracyChance::new_single(20, false)
        };
        Box::new(Decadent{
            v_basic_skills: Vec::new(),
            literacy_chance
        })
    }
}
