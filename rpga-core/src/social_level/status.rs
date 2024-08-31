use rpga_traits::Modifiered;

use crate::culture::Culture;

use super::{nobility::Nobility, wealth::Wealth};

/**
 Status, wealth, etc. contained within.
 */
pub struct Status {
    wealth: Wealth,
    nobility: Option<Nobility>,
}

impl Status {
    /**
     Generate a random (culturally appropriate) status level.

     **Params**
     * `culture` - one or other [Culture].
     
     **Returns** a [Status].
     */
    pub fn random(culture: &Culture) -> Self {
        let nobility = Nobility::random(culture);
        let wealth = Wealth::random(culture, if let Some(n) = &nobility {n.modifier()} else {0});

        Self { wealth, nobility }
    }
}

impl Modifiered for Status {
    /// Get ***SolMod***.
    fn modifier(&self) -> i32 {
        if let Some(_) = &self.nobility {
            self.wealth.modifier() + 5
        } else {
            self.wealth.modifier()
        }
    }
}
