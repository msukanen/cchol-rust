use crate::{skill::environment::EnvironmentType, race::Race, dice::DiceExt};

use super::CultureType;

pub trait Culture {
    fn level(&self) -> CultureType;
    fn cumod(&self) -> i32;
    fn native_env(&self) -> EnvironmentType;
    fn survival_rank(&self, env:EnvironmentType) -> i32;
}

pub struct CultureFactory;

impl CultureFactory {
    fn new_any() -> Box<dyn Culture> {
        match 1.d(10) {
            1 => Primitive::new(),
            2|3 => Nomad::new(),
            4|5|6 => Barbarian::new(),
            7|8|9 => Civilized::new(),
            _ => Decadent::new()
        }
    }

    pub fn new(race:&dyn Race) -> Box<dyn Culture> {
        let mut c: Box<dyn Culture>;
        loop {
            c = CultureFactory::new_any();
            if c.level() <= race.max_culture() {break;}
        } c
    }
}

struct Primitive;
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
}
impl Primitive {
    pub fn new() -> Box<dyn Culture> {Box::new(Primitive{})}
}

struct Nomad;
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
}
impl Nomad {
    pub fn new() -> Box<dyn Culture> {Box::new(Nomad{})}
}

struct Barbarian {
    nat_env: EnvironmentType
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
}
impl Barbarian {
    pub fn new() -> Box<dyn Culture> {
        Box::new(Barbarian{
            nat_env: if 1.d(10)<6 {EnvironmentType::WILDERNESS} else {EnvironmentType::URBAN}
        })
    }
}

struct Civilized {
    nat_env: EnvironmentType
}
impl Culture for Civilized {
    fn level(&self) -> CultureType { CultureType::CIVILIZED }
    fn cumod(&self) -> i32 { 2 }
    fn native_env(&self) -> EnvironmentType { self.nat_env }
    fn survival_rank(&self, env:EnvironmentType) -> i32 {
        match env {
            x if self.nat_env == x => 2,
            _ => 0
        }
    }
}
impl Civilized {
    pub fn new() -> Box<dyn Culture> {
        Box::new(Civilized{
            nat_env: if 1.d(10)<4 {EnvironmentType::WILDERNESS} else {EnvironmentType::URBAN}
        })
    }
}

struct Decadent;
impl Culture for Decadent {
    fn level(&self) -> CultureType { CultureType::DECADENT }
    fn cumod(&self) -> i32 { 2 }
    fn native_env(&self) -> EnvironmentType { EnvironmentType::URBAN }
    fn survival_rank(&self, env:EnvironmentType) -> i32 {
        match env {
            EnvironmentType::URBAN => 3,
            _ => 1
        }
    }
}
impl Decadent {
    pub fn new() -> Box<dyn Culture> {Box::new(Decadent{})}
}
