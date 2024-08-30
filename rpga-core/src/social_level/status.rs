use super::{nobility::Nobility, wealth::Wealth};

pub struct Status {
    wealth: Wealth,
    nobility: Option<Nobility>,
}

