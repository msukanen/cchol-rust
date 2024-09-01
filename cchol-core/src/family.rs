use rpga_generic::gender::Gender;

#[derive(Debug, Clone)]
pub enum Family {
    Mother,
    Father,
    MotherAndFather,
    Extended(i32, i32, i32),
    Grandparents(Gender)
}
