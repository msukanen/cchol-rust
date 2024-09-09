/// Various attribute types.
pub enum AttribType {
    STR, DEX, INT, CON, CHA, APP, MAG, Age,
}

/// Attribute container.
pub struct Attrib {
    r#type: AttribType,
    value: i32,
}

impl Attrib {
    pub fn new(t: AttribType) -> Self {
        // 10 *should* be kinda a-ok default for most values?
        let value = match t {
            AttribType::APP |
            AttribType::CHA |
            AttribType::CON |
            AttribType::DEX |
            AttribType::INT |
            AttribType::STR => 10,
            AttribType::MAG |
            AttribType::Age => 0,
        };
        Self { r#type: t, value }
    }

    pub fn new_valued(r#type: AttribType, value: i32) -> Self {
        Self { r#type, value }
    }
}
