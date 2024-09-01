use dicebag::{lo, DiceExt, HiLo};
use rpga_generic::gender::Gender;
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
    Guardian,
    None1,
    Orphanage,
}

/// Family specs.
pub struct Family {
    structure: FamilyStructure,
    adopted: bool,
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
                20 => FamilyStructure::Guardian,
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

        Self { structure, adopted }
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
}
