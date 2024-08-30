use rpga_traits::Modifiered;
use title::Title;

use crate::culture::Culture;

pub mod title;

pub struct Nobility {
    title: Title,
    timod: i32,
}

impl Modifiered for Nobility {
    /// Get ***TiMod***.
    fn modifier(&self) -> i32 {
        self.timod
    }
}

impl Nobility {
    pub fn random(culture: &Culture) -> Self {
        let title = Title::random(culture);
        match title {
            Title::Prince => {
                let mut prince_parent_title = Title::random(culture);
                while prince_parent_title == Title::Prince {
                    prince_parent_title = Title::random(culture)
                }
                let timod = prince_parent_title.random_timod(None);
                let timod= title.random_timod(Some(&timod));
                Self { title, timod }
            },
            _ => Self { timod: title.random_timod(None), title }
        }
    }
}
