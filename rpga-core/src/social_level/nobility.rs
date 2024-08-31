use dicebag::DiceExt;
use rpga_traits::{Modifiered, Named};
use title::Title;

use crate::culture::Culture;

pub mod title;

pub struct Nobility {
    title: Title,
    timod: i32,
    land_titles: Vec<String>,
    land_holdings: Option<i32>,
}

impl Modifiered for Nobility {
    /// Get ***TiMod***.
    fn modifier(&self) -> i32 {
        self.timod
    }
}

impl Nobility {
    /**
     Generate random (culturally appropriate) nobility thingy.

     **Params**
     * *culture* - a [Culture] reference.
     
     **Returns** some [Nobility] data.
     */
    pub fn random(culture: &Culture) -> Self {
        let title = Title::random(culture);
        match title {
            Title::Prince => {
                if 1.d100() < 21 {
                    let timod = 4.d10();
                    let land_holdings = title.random_land_holdings(Some(true));
                    Self { title, timod, land_titles: vec![], land_holdings }
                } else {
                    let mut prince_parent_title = Title::random(culture);
                    while prince_parent_title == Title::Prince {
                        prince_parent_title = Title::random(culture)
                    }
                    let timod = prince_parent_title.random_timod(None);
                    let timod= title.random_timod(Some(&timod));
                    Self { title, timod, land_titles: vec![], land_holdings: None }
                }
            },
            _ => Self {
                timod: title.random_timod(None),
                land_titles: title.random_land_titles(),
                land_holdings: title.random_land_holdings(None),
                title,
            }
        }
    }
}

impl Named for Nobility {
    fn name(&self) -> &str {
        self.title.as_str()
    }
}
