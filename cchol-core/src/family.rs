use dicebag::{lo, DiceExt, HiLo};
use rpga_generic::{birth::order::BirthOrder, gender::Gender};
use rpga_traits::Modifiered;

use crate::{culture::Culture, social_level::{status::Status, wealth::{Wealth, WealthRank}}};

/// Various family structures.
//
// NOTE: keep the ordering as-is!
//
#[derive(Debug, Clone, PartialEq)]
pub enum FamilyStructure {
    Mother,
    Father,
    MotherAndFather,
    Extended { grandparents: i32, aunts_and_uncles: i32, cousins: i32 },
    Grandparents { relation: Gender },
    Grandparent { gender: Gender, relation: Gender },
    Aunt { relation: Gender },
    Uncle { relation: Gender },
    AuntAndUncle { relation: Gender },
    //--past this, recheck if Guardian was rolled
    Guardian,//TODO: generate Guardian specs, T#754
    None1,
    Orphanage,
}

/// Family specs.
pub struct Family {
    structure: FamilyStructure,
    adopted: bool,
    siblings: Vec<(bool, Gender)>,
    birth_order: BirthOrder,
}

impl Family {
    /// Generate a random [Family].
    pub fn random(culture: &Culture) -> Self {
        fn choice(culture: &Culture) -> FamilyStructure {
            match 1.d20() + culture.modifier() {
                ..=8 => FamilyStructure::MotherAndFather,
                ..=12 => FamilyStructure::Extended{ grandparents: 1.d4(), aunts_and_uncles: 1.d4(), cousins: 1.d4()},
                13 => FamilyStructure::Grandparents { relation: Gender::maternal_or_paternal() },
                14 => FamilyStructure::Grandparent { gender: Gender::random(), relation: Gender::maternal_or_paternal() },
                15 => FamilyStructure::AuntAndUncle { relation: Gender::maternal_or_paternal() },
                16 => if lo!() {
                    FamilyStructure::Aunt { relation: Gender::maternal_or_paternal() }
                } else {
                    FamilyStructure::Uncle { relation: Gender::maternal_or_paternal() }
                },
                ..=18 => FamilyStructure::Mother,
                19 => FamilyStructure::Father,
                20 => FamilyStructure::Guardian,//TODO: Guardian
                ..=24 => FamilyStructure::None1,
                _ => FamilyStructure::Orphanage
            }
        }

        let mut structure = choice(culture);
        let mut adopted = false;
        if structure == FamilyStructure::Guardian {
            // See if actually a Guardian or if adopted instead.
            if 1.d20() > 8 {
                adopted = true;
                while match structure {
                    FamilyStructure::Guardian  |
                    FamilyStructure::None1     |
                    FamilyStructure::Orphanage => true,
                    _ => false
                } {
                    structure = choice(culture)
                }
            }
        }

        let mut siblings = vec![];
        loop {
            match 1.d20() {
                ..=2 => (),
                ..=9 => for _ in 0..1.d3() {siblings.push((false, Gender::random()))},
                ..=15 => for _ in 0..1.d3()+1 {siblings.push((false, Gender::random()))},
                ..=17 => for _ in 0..1.d4()+2 {siblings.push((false, Gender::random()))},
                ..=19 => for _ in 0..2.d4() {siblings.push((false, Gender::random()))},
                _ => {
                    // 1-3 illegitimate siblings.
                    for _ in 0..1.d3() {siblings.push((true, Gender::random()))};
                    continue;
                }
            }
            break;
        }

        Self {
            structure, adopted,
            birth_order: BirthOrder::random(siblings.len()),
            siblings
        }
    }

    /// Readjust status, if needed.
    pub fn readjust_status(&self, mut status: Status) -> Status {
        match self.structure {
            FamilyStructure::None1 => {
                let mut w = Wealth::from(WealthRank::Destitute);
                w.survival_mod += 1.d3();
                status.set_wealth(w);
                status
            },
            FamilyStructure::Orphanage => {
                status.set_wealth(Wealth::from(WealthRank::Poor));
                status
            },
            _ => status
        }
    }

    /// Get siblings' gender vector, if any exist.
    pub fn siblings(&self) -> &Vec<(bool, Gender)> {
        &self.siblings
    }

    /// Get birth order.
    pub fn birth_order(&self) -> &BirthOrder {
        &self.birth_order
    }

    /// Get adoption status.
    pub fn adopted(&self) -> bool {
        self.adopted
    }
}
