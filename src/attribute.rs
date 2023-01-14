use std::cmp::max;

pub trait Attribute {
    fn rank(&self) -> i32;
    fn adjust(&mut self, amount:i32) -> i32;
}

pub struct Strength { rank:i32 }
impl Strength {
    pub fn new(rank:i32) -> Self { Strength{ rank } }
}
pub struct Intelligence { rank:i32 }
pub struct Dexterity { rank:i32 }
impl Dexterity {
    pub fn new(rank:i32) -> Self { Dexterity { rank } }
}
pub struct MagicAbility { rank:i32 }
pub struct Constitution { rank:i32 }
impl Constitution {
    pub fn new(rank:i32) -> Self { Constitution { rank } }
}
pub struct Charisma { rank:i32 }
pub struct Appearance { rank:i32 }
pub struct Age { rank:i32 }

impl Attribute for Strength {
    fn rank(&self) -> i32 { self.rank }
    fn adjust(&mut self, amount:i32) -> i32 { self.rank = max(self.rank + amount, 0); self.rank }
}

impl Attribute for Intelligence {
    fn rank(&self) -> i32 { self.rank }
    fn adjust(&mut self, amount:i32) -> i32 { self.rank = max(self.rank + amount, 0); self.rank }
}

impl Attribute for Dexterity {
    fn rank(&self) -> i32 { self.rank }
    fn adjust(&mut self, amount:i32) -> i32 { self.rank = max(self.rank + amount, 0); self.rank }
}

impl Attribute for MagicAbility {
    fn rank(&self) -> i32 { self.rank }
    fn adjust(&mut self, amount:i32) -> i32 { self.rank = max(self.rank + amount, 0); self.rank }
}

impl Attribute for Constitution {
    fn rank(&self) -> i32 { self.rank }
    fn adjust(&mut self, amount:i32) -> i32 { self.rank = max(self.rank + amount, 0); self.rank }
}

impl Attribute for Charisma {
    fn rank(&self) -> i32 { self.rank }
    fn adjust(&mut self, amount:i32) -> i32 { self.rank += amount; self.rank }
}

impl Attribute for Appearance {
    fn rank(&self) -> i32 { self.rank }
    fn adjust(&mut self, amount:i32) -> i32 { self.rank += amount; self.rank }
}

impl Attribute for Age {
    fn rank(&self) -> i32 { self.rank }
    fn adjust(&mut self, amount:i32) -> i32 { self.rank = max(self.rank + amount, 0); self.rank }
}
