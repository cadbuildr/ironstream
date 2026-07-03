// FILE: b_rep_fill_acr_law.rs
// occt: BRepFill_ACRLaw

#[derive(Clone, Debug)]
pub struct BRepFillACRLaw;

impl BRepFillACRLaw {
    pub fn new() -> Self { BRepFillACRLaw }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _ = BRepFillACRLaw::new();
    }
}
