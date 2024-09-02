pub mod shape;

use dicebag::DiceExt;
use shape::BirthmarkShape;

use crate::{body_location::BodyLocation, color::Color};

/// Birthmark data goes here...
pub struct Birthmark {
    color: Color,
    shape: BirthmarkShape,
    location: BodyLocation,
}

impl Birthmark {
    /// Generate a random birthmark.
    pub fn random() -> Self {
        Self {
            color: if 1.d20() > 19 {Color::random()} else {Color::Natural},
            location: BodyLocation::random(),
            shape: BirthmarkShape::random(),
        }
    }
}
